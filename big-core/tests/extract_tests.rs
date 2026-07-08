use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use big_core::{extract_entry_to_path, parse_archive};

fn write_test_big_with_payload(path: &PathBuf) -> std::io::Result<()> {
    // Build a native BIGF archive with two payloads.
    let payload1 = b"AAAAAAAAAA"; // 10 bytes
    let payload2 = b"BBBBBBBBBBBBBBBBBBBB"; // 20 bytes

    let name1 = b"file1.bin";
    let name2 = b"file2.bin";

    // compute file headers region length
    let file_headers_region = (4 + 4 + name1.len() + 1) + (4 + 4 + name2.len() + 1);
    let header_size = 16 + file_headers_region as u64 + 8;
    let payload_start = header_size;

    let offset1 = payload_start;
    let offset2 = payload_start + payload1.len() as u64;
    let archive_size = payload_start + payload1.len() as u64 + payload2.len() as u64;

    let mut v = Vec::new();
    v.extend_from_slice(b"BIGF");
    v.extend_from_slice(&(archive_size as u32).to_le_bytes());
    v.extend_from_slice(&(2u32).to_be_bytes());
    v.extend_from_slice(&(header_size as u32).to_be_bytes());

    // entry1: offset (BE u32), length (BE u32), name, null
    v.extend_from_slice(&(offset1 as u32).to_be_bytes());
    v.extend_from_slice(&(payload1.len() as u32).to_be_bytes());
    v.extend_from_slice(name1);
    v.push(0u8);

    // entry2
    v.extend_from_slice(&(offset2 as u32).to_be_bytes());
    v.extend_from_slice(&(payload2.len() as u32).to_be_bytes());
    v.extend_from_slice(name2);
    v.push(0u8);

    // trailing 8 bytes
    v.extend_from_slice(&[0u8; 8]);

    // payloads
    v.extend_from_slice(payload1);
    v.extend_from_slice(payload2);

    let mut f = File::create(path)?;
    f.write_all(&v)?;
    f.sync_all()?;
    Ok(())
}

#[test]
fn extract_payloads() {
    let mut p = std::env::temp_dir();
    p.push("test_big_extract.big");
    let _ = std::fs::remove_file(&p);
    write_test_big_with_payload(&p).expect("write test big");

    let (_archive, _index, entries) = parse_archive(&p).expect("parse");
    assert_eq!(entries.len(), 2);

    let mut out1 = std::env::temp_dir();
    out1.push("out1.bin");
    let _ = fs::remove_file(&out1);
    extract_entry_to_path(&p, &entries[0], &out1).expect("extract1");
    let data1 = std::fs::read(&out1).expect("read1");
    assert_eq!(data1.len(), entries[0].length as usize);

    let mut out2 = std::env::temp_dir();
    out2.push("out2.bin");
    let _ = fs::remove_file(&out2);
    extract_entry_to_path(&p, &entries[1], &out2).expect("extract2");
    let data2 = std::fs::read(&out2).expect("read2");
    assert_eq!(data2.len(), entries[1].length as usize);
}

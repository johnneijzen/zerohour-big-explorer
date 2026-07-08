use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use big_core::{extract::stream_entry_to_writer, parser::parse_archive, paths::safe_join};

fn write_simple_big(path: &PathBuf) -> std::io::Result<()> {
    // Build a native BIGF archive with a single payload
    let payload = b"HELLOWORLD"; // 10 bytes
    let name = b"greeting.txt";

    let file_headers_region = 4 + 4 + name.len() + 1;
    let header_size = 16 + file_headers_region as u64 + 8;
    let payload_start = header_size;
    let offset = payload_start;
    let archive_size = payload_start + payload.len() as u64;

    let mut v = Vec::new();
    v.extend_from_slice(b"BIGF");
    v.extend_from_slice(&(archive_size as u32).to_le_bytes());
    v.extend_from_slice(&(1u32).to_be_bytes());
    v.extend_from_slice(&(header_size as u32).to_be_bytes());

    // index entry: offset (BE u32), length (BE u32), name, null
    v.extend_from_slice(&(offset as u32).to_be_bytes());
    v.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    v.extend_from_slice(name);
    v.push(0u8);

    v.extend_from_slice(&[0u8; 8]);

    v.extend_from_slice(payload);

    let mut f = File::create(path)?;
    f.write_all(&v)?;
    f.sync_all()?;
    Ok(())
}

#[test]
fn test_stream_entry_to_writer_reads_bytes() {
    let mut p = std::env::temp_dir();
    p.push("test_stream.big");
    let _ = std::fs::remove_file(&p);
    write_simple_big(&p).expect("write simple big");

    let (_meta, _index, entries) = parse_archive(&p).expect("parse");
    assert_eq!(entries.len(), 1);

    let mut file = File::open(&p).expect("open archive");
    let mut out = Vec::new();
    stream_entry_to_writer(&mut file, &entries[0], &mut out).expect("stream");
    assert_eq!(out.len(), entries[0].length as usize);
    assert_eq!(&out[..], b"HELLOWORLD");
}

#[test]
fn test_safe_join_rejects_parent_dir() {
    let base = std::env::temp_dir();
    let res = safe_join(&base, "../etc/passwd");
    assert!(res.is_err());
}

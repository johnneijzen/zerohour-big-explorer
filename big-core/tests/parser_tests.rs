use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use big_core::parse_archive;

fn write_test_big(path: &PathBuf) -> std::io::Result<()> {
    // Build a native BIGF archive with two simple entries
    let payload1 = vec![0u8; 10];
    let payload2 = vec![0u8; 200];
    let name1 = b"readme.txt";
    let name2 = b"image.png";

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

    v.extend_from_slice(&(offset1 as u32).to_be_bytes());
    v.extend_from_slice(&(payload1.len() as u32).to_be_bytes());
    v.extend_from_slice(name1);
    v.push(0u8);

    v.extend_from_slice(&(offset2 as u32).to_be_bytes());
    v.extend_from_slice(&(payload2.len() as u32).to_be_bytes());
    v.extend_from_slice(name2);
    v.push(0u8);

    v.extend_from_slice(&[0u8; 8]);

    v.extend_from_slice(&payload1);
    v.extend_from_slice(&payload2);

    let mut f = File::create(path)?;
    f.write_all(&v)?;
    f.sync_all()?;
    Ok(())
}

#[test]
fn parses_test_big() {
    let mut p = std::env::temp_dir();
    p.push("test_big_file.big");
    let _ = std::fs::remove_file(&p);
    write_test_big(&p).expect("write test file");

    let (_archive, index, entries) = parse_archive(&p).expect("parse archive");
    assert_eq!(index.entries_count, 2);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "readme.txt");
    assert_eq!(entries[1].name, "image.png");
}

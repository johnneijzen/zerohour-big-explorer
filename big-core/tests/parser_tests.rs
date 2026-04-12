use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use big_core::parse_archive;

fn write_test_big(path: &PathBuf) -> std::io::Result<()> {
    let mut v = Vec::new();
    // header: magic, version, index_offset, index_count
    v.extend_from_slice(b"BIG\0");
    v.extend_from_slice(&1u32.to_le_bytes());

    // index_offset = header_len (4+4+8+4 = 20)
    let index_offset = 20u64;
    v.extend_from_slice(&index_offset.to_le_bytes());

    // index_count = 2
    v.extend_from_slice(&2u32.to_le_bytes());

    // entry 1: name_len, name, offset, length, compressed, type_len, type
    let name1 = b"readme.txt";
    v.extend_from_slice(&(name1.len() as u16).to_le_bytes());
    v.extend_from_slice(name1);
    v.extend_from_slice(&0u64.to_le_bytes()); // offset
    v.extend_from_slice(&10u64.to_le_bytes()); // length
    v.push(0u8); // compressed
    v.push(4u8);
    v.extend_from_slice(b"text");

    // entry 2
    let name2 = b"image.png";
    v.extend_from_slice(&(name2.len() as u16).to_le_bytes());
    v.extend_from_slice(name2);
    v.extend_from_slice(&10u64.to_le_bytes()); // offset
    v.extend_from_slice(&200u64.to_le_bytes()); // length
    v.push(1u8); // compressed
    v.push(5u8);
    v.extend_from_slice(b"image");

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

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use big_core::{extract::stream_entry_to_writer, parser::parse_archive, paths::safe_join};

fn write_simple_big(path: &PathBuf) -> std::io::Result<()> {
    let mut v = Vec::new();
    v.extend_from_slice(b"BIG\0");
    v.extend_from_slice(&1u32.to_le_bytes());

    // placeholder index_offset
    let index_offset_pos = v.len();
    v.extend_from_slice(&0u64.to_le_bytes());
    v.extend_from_slice(&1u32.to_le_bytes()); // one entry

    // build index
    let mut idx = Vec::new();
    let payload = b"HELLOWORLD"; // 10 bytes
    let name = b"greeting.txt";
    idx.extend_from_slice(&(name.len() as u16).to_le_bytes());
    idx.extend_from_slice(name);
    idx.extend_from_slice(&0u64.to_le_bytes()); // offset placeholder
    idx.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    idx.push(0u8);
    idx.push(0u8);

    let index_offset = v.len() as u64;
    let index_offset_bytes = index_offset.to_le_bytes();
    v[index_offset_pos..(index_offset_pos + 8)].copy_from_slice(&index_offset_bytes);
    v.extend_from_slice(&idx);

    // payload start
    let payload_offset = v.len() as u64;
    v.extend_from_slice(payload);

    // patch offset in index
    let mut cursor = index_offset as usize;
    let name_len = u16::from_le_bytes([v[cursor], v[cursor + 1]]) as usize;
    cursor += 2 + name_len;
    v[cursor..(cursor + 8)].copy_from_slice(&payload_offset.to_le_bytes());

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

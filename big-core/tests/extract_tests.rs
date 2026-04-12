use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use big_core::{extract_entry_to_path, parse_archive};

fn write_test_big_with_payload(path: &PathBuf) -> std::io::Result<()> {
    let mut v = Vec::new();
    // header: magic, version, index_offset, index_count
    v.extend_from_slice(b"BIG\0");
    v.extend_from_slice(&1u32.to_le_bytes());

    // placeholder index_offset; we'll compute after building index
    let index_offset_pos = v.len();
    v.extend_from_slice(&0u64.to_le_bytes());

    // index_count = 2
    v.extend_from_slice(&2u32.to_le_bytes());

    // build index in separate buffer
    let mut idx = Vec::new();

    // payloads
    let payload1 = b"AAAAAAAAAA"; // 10 bytes
    let payload2 = b"BBBBBBBBBBBBBBBBBBBB"; // 20 bytes

    // entry1 metadata
    let name1 = b"file1.bin";
    idx.extend_from_slice(&(name1.len() as u16).to_le_bytes());
    idx.extend_from_slice(name1);
    let offset1 = 0u64; // placeholder
    idx.extend_from_slice(&offset1.to_le_bytes());
    idx.extend_from_slice(&(payload1.len() as u64).to_le_bytes());
    idx.push(0u8);
    idx.push(0u8); // type len

    // entry2 metadata
    let name2 = b"file2.bin";
    idx.extend_from_slice(&(name2.len() as u16).to_le_bytes());
    idx.extend_from_slice(name2);
    let offset2 = 0u64; // placeholder
    idx.extend_from_slice(&offset2.to_le_bytes());
    idx.extend_from_slice(&(payload2.len() as u64).to_le_bytes());
    idx.push(0u8);
    idx.push(0u8);

    // compute index_offset: current header length
    let index_offset = v.len() as u64;
    // replace placeholder
    let index_offset_bytes = index_offset.to_le_bytes();
    for i in 0..8 { v[index_offset_pos + i] = index_offset_bytes[i]; }

    // append index
    v.extend_from_slice(&idx);

    // payload offsets start here
    let payloads_start = v.len() as u64;
    // set offsets in index (we know their positions: first entry offset is at index_offset + name1_len+2? easier to rebuild )

    // For simplicity, append payloads and then rewrite the offsets directly in the buffer.
    let payload1_offset = v.len() as u64;
    v.extend_from_slice(payload1);
    let payload2_offset = v.len() as u64;
    v.extend_from_slice(payload2);

    // Now patch offsets in the index area: find offsets by scanning
    let mut cursor = index_offset as usize;
    // entry1: name_len(2)
    let name1_len = u16::from_le_bytes([v[cursor], v[cursor+1]]) as usize; cursor += 2;
    cursor += name1_len; // name
    // write payload1_offset
    let off_bytes = payload1_offset.to_le_bytes();
    for i in 0..8 { v[cursor + i] = off_bytes[i]; }
    cursor += 8;
    // skip length (8) + compressed(1) + type_len(1)
    cursor += 8 + 1 + 1;

    // entry2: name_len
    let name2_len = u16::from_le_bytes([v[cursor], v[cursor+1]]) as usize; cursor += 2;
    cursor += name2_len;
    // write payload2_offset
    let off_bytes2 = payload2_offset.to_le_bytes();
    for i in 0..8 { v[cursor + i] = off_bytes2[i]; }

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

    let mut out1 = std::env::temp_dir(); out1.push("out1.bin"); let _ = fs::remove_file(&out1);
    extract_entry_to_path(&p, &entries[0], &out1).expect("extract1");
    let data1 = std::fs::read(&out1).expect("read1");
    assert_eq!(data1.len(), entries[0].length as usize);

    let mut out2 = std::env::temp_dir(); out2.push("out2.bin"); let _ = fs::remove_file(&out2);
    extract_entry_to_path(&p, &entries[1], &out2).expect("extract2");
    let data2 = std::fs::read(&out2).expect("read2");
    assert_eq!(data2.len(), entries[1].length as usize);
}

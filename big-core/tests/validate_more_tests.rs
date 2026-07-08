use std::fs::File;
use std::io::{Seek, Write};
use tempfile::tempdir;

#[test]
fn validate_accepts_packed_archive() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("srcdir");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("a.txt"), b"hello").unwrap();
    std::fs::write(src.join("b.txt"), b"world!!!").unwrap();

    let out = dir.path().join("out.big");
    let job = big_core::models::RepackJob {
        source_dir: format!("{}:{}", src.display(), out.display()),
        compression: None,
    };
    // run repack (uses packer)
    let _ = big_core::pack::run_repack(&job).unwrap();

    let res = big_core::validate::validate_archive(&out).unwrap();
    assert!(res.errors.is_empty(), "expected no errors, got: {:?}", res.errors);
}

#[test]
fn validate_detects_overlapping_entries() {
    // craft a minimal archive binary with 2 entries that overlap
    let dir = tempdir().unwrap();
    let p = dir.path().join("ovr.big");
    let mut f = File::create(&p).unwrap();
    // Create a BIGF archive where two entries overlap (offset/length chosen accordingly)
    let name1 = b"x";
    let name2 = b"y";
    let file_headers_region = (4 + 4 + name1.len() + 1) + (4 + 4 + name2.len() + 1);
    let header_size = 16 + file_headers_region as u64 + 8;
    let payload_start = header_size;

    // Choose offsets that overlap: entry1 at 100..150, entry2 at 120..180
    let entry1_offset = 100u64;
    let entry1_len = 50u64;
    let entry2_offset = 120u64;
    let entry2_len = 60u64;

    // compute archive size large enough to include payload region
    let archive_size = payload_start.max(entry2_offset + entry2_len + 10);

    // write header
    f.write_all(b"BIGF").unwrap();
    f.write_all(&(archive_size as u32).to_le_bytes()).unwrap();
    f.write_all(&(2u32).to_be_bytes()).unwrap();
    f.write_all(&(header_size as u32).to_be_bytes()).unwrap();

    // entry1
    f.write_all(&(entry1_offset as u32).to_be_bytes()).unwrap();
    f.write_all(&(entry1_len as u32).to_be_bytes()).unwrap();
    f.write_all(name1).unwrap();
    f.write_all(&[0u8]).unwrap();

    // entry2
    f.write_all(&(entry2_offset as u32).to_be_bytes()).unwrap();
    f.write_all(&(entry2_len as u32).to_be_bytes()).unwrap();
    f.write_all(name2).unwrap();
    f.write_all(&[0u8]).unwrap();

    // trailing 8 bytes
    f.write_all(&[0u8; 8]).unwrap();

    // pad file until archive_size with zeros
    let cur = f.stream_position().unwrap();
    if cur < archive_size {
        let pad = vec![0u8; (archive_size - cur) as usize];
        f.write_all(&pad).unwrap();
    }
    f.flush().unwrap();

    let res = big_core::validate::validate_archive(&p).unwrap();
    assert!(!res.errors.is_empty(), "expected overlap errors, got none");
}

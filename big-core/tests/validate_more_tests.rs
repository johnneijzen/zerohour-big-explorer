use std::fs::{File, write};
use std::io::{Seek, SeekFrom, Write};
use tempfile::tempdir;

#[test]
fn validate_accepts_packed_archive() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("srcdir");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("a.txt"), b"hello").unwrap();
    std::fs::write(src.join("b.txt"), b"world!!!").unwrap();

    let out = dir.path().join("out.big");
    let job = big_core::models::RepackJob { source_dir: format!("{}:{}", src.display(), out.display()), compression: None };
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

    // header: magic, version, index_offset (20), index_count (2)
    f.write_all(b"BIG\0").unwrap();
    f.write_all(&1u32.to_le_bytes()).unwrap();
    f.write_all(&20u64.to_le_bytes()).unwrap();
    f.write_all(&2u32.to_le_bytes()).unwrap();

    // index (2 entries)
    // entry1: name 'x' len=1, offset=100, length=50, compressed=0, type_len=0
    f.write_all(&1u16.to_le_bytes()).unwrap();
    f.write_all(b"x").unwrap();
    f.write_all(&100u64.to_le_bytes()).unwrap();
    f.write_all(&50u64.to_le_bytes()).unwrap();
    f.write_all(&[0u8]).unwrap();
    f.write_all(&[0u8]).unwrap();

    // entry2: name 'y' len=1, offset=120 (overlaps 100..150), length=60
    f.write_all(&1u16.to_le_bytes()).unwrap();
    f.write_all(b"y").unwrap();
    f.write_all(&120u64.to_le_bytes()).unwrap();
    f.write_all(&60u64.to_le_bytes()).unwrap();
    f.write_all(&[0u8]).unwrap();
    f.write_all(&[0u8]).unwrap();

    // pad until payload region and write dummy payloads
    let cur = f.seek(SeekFrom::Current(0)).unwrap();
    if cur < 200 {
        let pad = vec![0u8; (200 - cur) as usize];
        f.write_all(&pad).unwrap();
    }
    // write payloads enough to cover offsets
    let mut payload = vec![0u8; 200];
    f.write_all(&payload).unwrap();
    f.flush().unwrap();

    let res = big_core::validate::validate_archive(&p).unwrap();
    assert!(!res.errors.is_empty(), "expected overlap errors, got none");
}

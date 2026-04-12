use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn validate_detects_invalid_magic() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("bad.big");
    let mut f = File::create(&p).unwrap();
    f.write_all(b"NOTBIGHEADER").unwrap();
    let res = big_core::validate::validate_archive(&p).unwrap();
    assert!(!res.errors.is_empty());
}

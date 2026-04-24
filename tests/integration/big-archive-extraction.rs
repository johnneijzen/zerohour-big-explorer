use std::fs::{self, File};
use std::io::Write;

#[test]
fn pack_and_extract_roundtrip() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    let _ = std::fs::create_dir_all(&src);

    let sample = src.join("hello.txt");
    let mut f = File::create(&sample).expect("create sample");
    writeln!(f, "hello zerohour").expect("write sample");

    let archive = tmp.path().join("out.big");

    big_core::pack::pack_directory(&src, &archive).expect("pack directory");

    let (_meta, _index, entries) = big_core::parser::parse_archive(&archive).expect("parse");
    assert!(entries.iter().any(|e| e.name.ends_with("hello.txt")));

    let entry = entries.into_iter().find(|e| e.name.ends_with("hello.txt")).unwrap();
    let bytes = big_core::extract::extract_file(&archive, &entry).expect("extract_file");
    assert!(bytes.len() > 0);

    let outdir = tmp.path().join("out");
    big_core::extract::extract_all(&archive, &outdir).expect("extract_all");

    let outtxt = outdir.join("hello.txt");
    let data = fs::read_to_string(outtxt).expect("read extracted");
    assert!(data.contains("hello zerohour"));
}

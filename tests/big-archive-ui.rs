use std::fs::{self, File};
use std::io::Write;

#[test]
fn tauri_extract_file_bytes_matches_core() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    let _ = std::fs::create_dir_all(&src);

    let sample = src.join("hello.txt");
    let mut f = File::create(&sample).expect("create sample");
    writeln!(f, "hello zerohour").expect("write sample");

    let archive = tmp.path().join("out.big");
    big_core::pack::pack_directory(&src, &archive).expect("pack directory");

    let (_meta, _index, entries) = big_core::parser::parse_archive(&archive).expect("parse");
    let entry = entries.into_iter().find(|e| e.name.ends_with("hello.txt")).unwrap();

    let tauri_bytes = big_tauri_lib::commands::extract_file_bytes(
        archive.to_string_lossy().to_string(),
        entry.name.clone(),
    )
    .expect("tauri extract");

    let core_bytes = big_core::extract::extract_file(&archive, &entry).expect("core extract");
    assert_eq!(tauri_bytes, core_bytes);
}

#[test]
fn tauri_unpack_all_writes_files() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    let _ = std::fs::create_dir_all(src.join("nested"));

    let sample = src.join("nested").join("hello2.txt");
    let mut f = File::create(&sample).expect("create sample");
    writeln!(f, "unpack test").expect("write sample");

    let archive = tmp.path().join("out2.big");
    big_core::pack::pack_directory(&src, &archive).expect("pack");

    let outdir = tmp.path().join("out");
    big_tauri_lib::commands::unpack_all(
        archive.to_string_lossy().to_string(),
        outdir.to_string_lossy().to_string(),
    )
    .expect("unpack_all");

    let extracted = outdir.join("nested").join("hello2.txt");
    let data = fs::read_to_string(extracted).expect("read extracted");
    assert!(data.contains("unpack test"));
}

// Integration tests for BIG archive UI commands (T014, T018)

use std::fs::File;
use std::io::Write;

#[test]
fn ui_loads_file_list() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    let _ = std::fs::create_dir_all(&src);

    let sample = src.join("hello_ui.txt");
    let mut f = File::create(&sample).expect("create sample");
    writeln!(f, "hello ui").expect("write sample");

    let archive = tmp.path().join("ui_list.big");
    big_core::pack::pack_directory(&src, &archive).expect("pack directory");

    let entries = big_tauri_lib::commands::list_archive(archive.to_string_lossy().to_string(), None)
        .expect("list archive");

    assert!(entries.len() > 0, "expected archived entries to be listed");
    assert!(entries.iter().any(|e| e.name.ends_with("hello_ui.txt")));
}

#[test]
fn ui_extract_bytes_matches_core_checksum() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    let _ = std::fs::create_dir_all(&src);

    let sample = src.join("hello_extract.txt");
    let mut f = File::create(&sample).expect("create sample");
    writeln!(f, "extract test").expect("write sample");

    let archive = tmp.path().join("ui_extract.big");
    big_core::pack::pack_directory(&src, &archive).expect("pack directory");

    let (_meta, _index, entries) = big_core::parser::parse_archive(&archive).expect("parse");
    let entry = entries.into_iter().find(|e| e.name.ends_with("hello_extract.txt")).unwrap();

    let tauri_bytes = big_tauri_lib::commands::extract_file_bytes(
        archive.to_string_lossy().to_string(),
        entry.name.clone(),
    )
    .expect("tauri extract");

    let core_bytes = big_core::extract::extract_file(&archive, &entry).expect("core extract");
    assert_eq!(tauri_bytes, core_bytes, "tauri bytes should match core extract bytes");
}

#[test]
fn pack_smoke() {
    // create temp dir with two files, pack it, and parse the resulting archive
    let mut dir = std::env::temp_dir(); dir.push("pack_test_src"); let _ = std::fs::remove_dir_all(&dir); std::fs::create_dir_all(&dir).unwrap();
    let mut f1 = dir.clone(); f1.push("a.txt"); std::fs::write(&f1, b"hello").unwrap();
    let mut f2 = dir.clone(); f2.push("sub/b.txt"); std::fs::create_dir_all(f2.parent().unwrap()).unwrap(); std::fs::write(&f2, b"world!").unwrap();

    let mut out = std::env::temp_dir(); out.push("packed_test.big"); let _ = std::fs::remove_file(&out);
    big_core::pack::pack_directory(&dir, &out).expect("pack");

    let (_archive, index, entries) = big_core::parse_archive(&out).expect("parse");
    assert_eq!(entries.len(), 2);
    assert!(entries.iter().any(|e| e.name.ends_with("a.txt")));
    assert!(entries.iter().any(|e| e.name.ends_with("b.txt")));
}

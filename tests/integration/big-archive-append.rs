use std::fs::File;
use std::io::Write;

#[test]
fn append_file_roundtrip_and_collision() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    let _ = std::fs::create_dir_all(&src);

    let a = src.join("a.txt");
    let mut f = File::create(&a).expect("create a");
    writeln!(f, "alpha").expect("write a");

    let archive = tmp.path().join("append.big");
    big_core::pack::pack_directory(&src, &archive).expect("pack");

    // create file to append
    let b = tmp.path().join("b.txt");
    let mut fb = File::create(&b).expect("create b");
    writeln!(fb, "beta").expect("write b");

    // append
    big_core::pack::append_file_to_archive(&archive, &b, "b.txt", false).expect("append b");

    let (_m, _i, entries) = big_core::parser::parse_archive(&archive).expect("parse");
    assert!(entries.iter().any(|e| e.name.ends_with("b.txt")));

    // try append again without force -> should error
    let res = big_core::pack::append_file_to_archive(&archive, &b, "b.txt", false);
    assert!(res.is_err());
}

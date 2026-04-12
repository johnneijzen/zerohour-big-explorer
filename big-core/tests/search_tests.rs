use big_core::models::Entry;
use big_core::search::filter_entries;

fn make_entry(name: &str) -> Entry {
    Entry { name: name.to_string(), offset: 0, length: 0, compressed: false, r#type: None }
}

#[test]
fn search_empty_query_returns_all() {
    let entries = vec![make_entry("foo/bar.txt"), make_entry("baz/qux.png")];
    let res = filter_entries(&entries, None);
    assert_eq!(res.len(), 2);
}

#[test]
fn search_substring_matches() {
    let entries = vec![
        make_entry("sound/music.ogg"),
        make_entry("textures/grass.png"),
        make_entry("docs/readme.txt"),
    ];
    let res = filter_entries(&entries, Some("music"));
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "sound/music.ogg");
}

#[test]
fn search_case_insensitive() {
    let entries = vec![make_entry("Sound/Music.OGG"), make_entry("textures/grass.png")];
    let res = filter_entries(&entries, Some("music"));
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "Sound/Music.OGG");
}

#[test]
fn search_path_segment_match() {
    let entries = vec![make_entry("maps/level1/data.bin"), make_entry("maps/level2/data.bin")];
    let res = filter_entries(&entries, Some("level2"));
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "maps/level2/data.bin");
}

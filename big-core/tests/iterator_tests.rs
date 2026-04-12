use big_core::EntryIterator;
use big_core::models::Entry;

#[test]
fn iterator_yields_entries_in_order() {
    let entries = vec![
        Entry {
            name: "a.txt".into(),
            offset: 0,
            length: 10,
            compressed: false,
            r#type: Some("text".into()),
        },
        Entry {
            name: "b.png".into(),
            offset: 10,
            length: 200,
            compressed: true,
            r#type: Some("image".into()),
        },
    ];

    let mut it = EntryIterator::from_entries(entries.clone());

    let first = it.next().expect("first entry");
    assert_eq!(first.name, "a.txt");

    let second = it.next().expect("second entry");
    assert_eq!(second.name, "b.png");

    assert!(it.next().is_none());
}

#[test]
fn resolve_metadata_returns_type() {
    let entry = Entry {
        name: "x.bin".into(),
        offset: 0,
        length: 4,
        compressed: false,
        r#type: Some("binary".into()),
    };
    let t = EntryIterator::resolve_metadata(&entry);
    assert_eq!(t.as_deref(), Some("binary"));
}

use crate::models::Entry;

/// A simple lazy iterator over `Entry` items.
/// This is a scaffold: real implementation should stream entries from
/// the archive file without materializing everything in memory.
#[derive(Debug)]
pub struct EntryIterator {
    entries: Vec<Entry>,
    pos: usize,
}

impl EntryIterator {
    pub fn from_entries(entries: Vec<Entry>) -> Self {
        Self { entries, pos: 0 }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Resolve additional metadata for an entry (placeholder).
    pub fn resolve_metadata(entry: &Entry) -> Option<String> {
        entry.r#type.clone()
    }
}

impl Iterator for EntryIterator {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.entries.len() {
            None
        } else {
            let e = self.entries[self.pos].clone();
            self.pos += 1;
            Some(e)
        }
    }
}

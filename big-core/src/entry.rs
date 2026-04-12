use crate::models::Entry;
use std::io::{Read, Seek};

pub struct EntryReader {
    // placeholder for state (e.g., decompressor)
}

impl EntryReader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_payload<R: Read + Seek>(
        &self,
        _r: &mut R,
        _entry: &Entry,
    ) -> anyhow::Result<Vec<u8>> {
        // Placeholder: real implementation should seek to entry.offset and read `entry.length` bytes
        Ok(Vec::new())
    }
}

impl Default for EntryReader {
    fn default() -> Self {
        Self::new()
    }
}

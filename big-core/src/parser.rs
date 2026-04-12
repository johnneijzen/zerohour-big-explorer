use anyhow::Context;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::archive;
use crate::index::IndexReader;
use crate::models::{Archive, Entry, Index};

/// Simple on-disk header format used by this scaffold:
/// [0..4) magic = b"BIG\0"
/// [4..8) u32 version (LE)
/// [8..16) u64 index_offset (LE)
/// [16..20) u32 index_count (LE)
pub fn parse_archive<P: AsRef<Path>>(path: P) -> anyhow::Result<(Archive, Index, Vec<Entry>)> {
    let path_ref = path.as_ref();

    let archive = archive::open(path_ref)
        .with_context(|| format!("opening archive {}", path_ref.display()))?;

    let mut f = File::open(path_ref)
        .with_context(|| format!("opening file {} for index read", path_ref.display()))?;

    // read header
    let mut magic = [0u8; 4];
    f.read_exact(&mut magic)?;
    if &magic != b"BIG\0" {
        // not a real BIG file according to our scaffold; but continue with empty index
        return Ok((archive, Index { entries_count: 0 }, Vec::new()));
    }

    let mut u32buf = [0u8; 4];
    f.read_exact(&mut u32buf)?;
    let _version = u32::from_le_bytes(u32buf);

    let mut u64buf = [0u8; 8];
    f.read_exact(&mut u64buf)?;
    let index_offset = u64::from_le_bytes(u64buf);

    f.read_exact(&mut u32buf)?;
    let index_count = u32::from_le_bytes(u32buf);

    // use IndexReader to read entries
    let reader = IndexReader::new();
    let entries = reader.read_entries_from(&mut f, index_offset, index_count).unwrap_or_default();

    Ok((archive, Index { entries_count: entries.len() }, entries))
}

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

    // Scaffold header: "BIG\0" with version/u64 index_offset/u32 index_count
    if &magic == b"BIG\0" {
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
        let entries =
            reader.read_entries_from(&mut f, index_offset, index_count).unwrap_or_default();

        return Ok((archive, Index { entries_count: entries.len() }, entries));
    }

    // Try project's native BIG format (C++ style header starting with "BIGF")
    if &magic == b"BIGF" {
        // SBigHeader: u32 signature, u32 bigFileSize, u32 fileCount, u32 headerSize
        let mut u32buf = [0u8; 4];
        f.read_exact(&mut u32buf)?;
        let _big_file_size = u32::from_le_bytes(u32buf);

        f.read_exact(&mut u32buf)?;
        let raw_file_count = u32::from_le_bytes(u32buf);

        f.read_exact(&mut u32buf)?;
        let raw_header_size = u32::from_le_bytes(u32buf);

        // Some fields in the native C++ header are byte-swapped in different builds.
        // Try both little-endian and swapped variants and pick the plausible one.
        let file_count_le = raw_file_count as usize;
        let file_count_swapped = raw_file_count.swap_bytes() as usize;

        let header_size_le = raw_header_size as usize;
        let header_size_swapped = raw_header_size.swap_bytes() as usize;

        // choose header_size and file_count that look plausible given archive size
        let archive_size = archive.size as usize;

        let mut file_count = file_count_le;
        let mut header_size = header_size_le;
        if !(header_size_le <= archive_size && file_count_le < 1_000_000)
            && header_size_swapped <= archive_size
            && file_count_swapped < 1_000_000
        {
            file_count = file_count_swapped;
            header_size = header_size_swapped;
        }

        // Calculate file headers region size: header_size - SBigHeader(16) - SBigLastHeader(8)
        let file_headers_region = header_size.saturating_sub(16 + 8);

        // Read the file headers region into buffer
        let mut fh_buf = vec![0u8; file_headers_region];
        if file_headers_region > 0 {
            f.read_exact(&mut fh_buf)?;
        }

        let mut cursor = std::io::Cursor::new(fh_buf);
        let mut entries = Vec::with_capacity(file_count);

        for _ in 0..file_count {
            // offset u32
            let mut offb = [0u8; 4];
            cursor.read_exact(&mut offb)?;
            // Stored with inverted byte order in native BIG format (utils::GetInvert)
            let offset = u32::from_le_bytes(offb).swap_bytes() as u64;

            // size u32
            let mut szb = [0u8; 4];
            cursor.read_exact(&mut szb)?;
            let length = u32::from_le_bytes(szb).swap_bytes() as u64;

            // read null-terminated name
            let mut name_bytes = Vec::new();
            loop {
                let mut b = [0u8; 1];
                if cursor.read_exact(&mut b).is_err() {
                    break;
                }
                if b[0] == 0 {
                    break;
                }
                name_bytes.push(b[0]);
            }
            let name = String::from_utf8_lossy(&name_bytes).into_owned();

            entries.push(Entry { name, offset, length, compressed: false, r#type: None });
        }

        return Ok((archive, Index { entries_count: entries.len() }, entries));
    }

    // Unknown header: not a supported BIG format, return empty index
    Ok((archive, Index { entries_count: 0 }, Vec::new()))
}

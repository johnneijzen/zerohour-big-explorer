use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::models::Entry;

/// Stream the payload for `entry` from `r` into `w`.
pub fn stream_entry_to_writer<R: Read + Seek, W: Write>(r: &mut R, entry: &Entry, mut w: W) -> anyhow::Result<()> {
    let mut remaining = entry.length;
    r.seek(SeekFrom::Start(entry.offset))?;

    let mut buf = [0u8; 8 * 1024];
    while remaining > 0 {
        let to_read = std::cmp::min(remaining, buf.len() as u64) as usize;
        r.read_exact(&mut buf[..to_read])?;
        w.write_all(&buf[..to_read])?;
        remaining -= to_read as u64;
    }

    Ok(())
}

/// Safely extract an entry from `archive_path` to `dest_path`.
/// Writes to a temporary `.part` file in the destination directory, then atomically renames.
pub fn extract_entry_to_path<P: AsRef<Path>>(archive_path: P, entry: &Entry, dest_path: P) -> anyhow::Result<()> {
    let archive_path = archive_path.as_ref();
    let dest_path = dest_path.as_ref();

    let mut f = File::open(archive_path)?;

    // create temporary file next to destination
    let mut tmp_path = dest_path.to_path_buf();
    if let Some(os) = tmp_path.file_name() {
        let mut tmp_name = os.to_os_string();
        tmp_name.push(".part");
        tmp_path.set_file_name(tmp_name);
    } else {
        return Err(anyhow::anyhow!("invalid destination path"));
    }

    let mut tmp = OpenOptions::new().create(true).write(true).truncate(true).open(&tmp_path)?;
    stream_entry_to_writer(&mut f, entry, &mut tmp)?;
    tmp.sync_all()?;

    fs::rename(&tmp_path, dest_path)?;
    Ok(())
}

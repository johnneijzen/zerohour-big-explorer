use crate::models::RepackJob;
use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// Pack a directory into a simple deterministic .BIG archive.
pub fn pack_directory<P: AsRef<Path>>(src: P, dest: P) -> anyhow::Result<()> {
    pack_directory_with_progress(src, dest, None)
}

/// Pack with optional progress sender.
pub fn pack_directory_with_progress<P: AsRef<Path>>(src: P, dest: P, progress: Option<crate::progress::ProgressSender>) -> anyhow::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    // collect files sorted by path
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in walkdir::WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    files.sort();

    // compute index entry sizes
    let mut index_size: u64 = 0;
    let mut names: Vec<String> = Vec::new();
    let mut lengths: Vec<u64> = Vec::new();
    for p in &files {
        let rel = p.strip_prefix(src).unwrap().to_string_lossy().to_string();
        let meta = fs::metadata(p)?;
        names.push(rel);
        lengths.push(meta.len());
        index_size += 2 + (names.last().unwrap().len() as u64) + 8 + 8 + 1 + 1; // name_len + name + offset + length + compressed + type_len
    }

    let header_len: u64 = 4 + 4 + 8 + 4; // magic + version + index_offset + index_count
    let index_offset = header_len; // index immediately after header
    let payload_start = index_offset + index_size;

    // compute offsets
    let mut offsets: Vec<u64> = Vec::new();
    let mut cur = payload_start;
    for len in &lengths {
        offsets.push(cur);
        cur += *len;
    }

    // write file
    let mut f = File::create(dest)?;
    // header
    f.write_all(b"BIG\0")?;
    f.write_all(&1u32.to_le_bytes())?; // version
    f.write_all(&index_offset.to_le_bytes())?;
    f.write_all(&(files.len() as u32).to_le_bytes())?;

    // write index
    for (i, name) in names.iter().enumerate() {
        let name_bytes = name.as_bytes();
        f.write_all(&(name_bytes.len() as u16).to_le_bytes())?;
        f.write_all(name_bytes)?;
        f.write_all(&offsets[i].to_le_bytes())?;
        f.write_all(&lengths[i].to_le_bytes())?;
        f.write_all(&[0u8])?; // compressed = false
        f.write_all(&[0u8])?; // type_len = 0
    }

    // write payloads
    for (idx, p) in files.iter().enumerate() {
        if let Some(ref tx) = progress {
            let _ = tx.send(crate::progress::Progress::Message(format!("Writing {}", p.display())));
            let _ = tx.send(crate::progress::Progress::Percent(((idx * 100) / files.len()) as u8));
        }
        let mut srcf = File::open(p)?;
        std::io::copy(&mut srcf, &mut f)?;
    }

    if let Some(ref tx) = progress {
        let _ = tx.send(crate::progress::Progress::Completed);
    }

    Ok(())
}

/// Run a repack job synchronously (returns job id placeholder after performing pack if dest provided in job.source_dir format `src->dest`).
pub fn run_repack(job: &RepackJob) -> anyhow::Result<String> {
    // Interpret job.source_dir as "src:dest" or just src (no-op)
    if job.source_dir.contains(":") {
        let parts: Vec<&str> = job.source_dir.splitn(2, ':').collect();
        let src = parts[0];
        let dest = parts[1];
        pack_directory(src, dest)?;
        Ok(format!("repack-job-{}->{}", src, dest))
    } else {
        Ok(format!("repack-job-{}", job.source_dir))
    }
}

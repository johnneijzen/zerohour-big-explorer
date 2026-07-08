use crate::models::RepackJob;
use std::fs::{self, File};
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

/// Pack a directory into a native SAGE/ZeroHour .BIG archive (BIGF format).
pub fn pack_directory<P: AsRef<Path>>(src: P, dest: P) -> anyhow::Result<()> {
    pack_directory_with_progress(src, dest, None)
}

/// Pack with optional progress sender. Produces a header compatible with the
/// native `BIGF`/`BIG4` layout used by SAGE/ZeroHour: FourCC, archive_size
/// (LE u32), file_count (BE u32), header_size (BE u32), file index entries
/// (offset BE u32, length BE u32, null-terminated name...), an 8-byte tail,
/// then the concatenated payloads.
pub fn pack_directory_with_progress<P: AsRef<Path>>(
    src: P,
    dest: P,
    progress: Option<crate::progress::ProgressSender>,
) -> anyhow::Result<()> {
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

    // build name and length lists and compute file-headers region size
    let mut names: Vec<String> = Vec::new();
    let mut lengths: Vec<u64> = Vec::new();
    let mut file_headers_region: u64 = 0;
    for p in &files {
        let rel = p.strip_prefix(src).unwrap().to_string_lossy().to_string();
        let meta = fs::metadata(p)?;
        let name_len = rel.len() as u64;
        // Each index entry: offset (4) + length (4) + name bytes + null (1)
        file_headers_region += 4 + 4 + name_len + 1;
        names.push(rel);
        lengths.push(meta.len());
    }

    // header_size = SBigHeader(16) + file_headers_region + SBigLastHeader(8)
    let header_size: u64 = 16 + file_headers_region + 8;
    let payload_start = header_size;

    // compute offsets (within archive) for each payload
    let mut offsets: Vec<u64> = Vec::new();
    let mut cur = payload_start;
    for len in &lengths {
        offsets.push(cur);
        cur += *len;
    }

    let archive_size = cur; // final archive size

    // write file
    let mut f = File::create(dest)?;
    // header: FourCC
    f.write_all(b"BIGF")?;
    // archive size stored as little-endian u32 per spec
    f.write_all(&(archive_size as u32).to_le_bytes())?;
    // file_count and header_size are big-endian
    f.write_all(&(files.len() as u32).to_be_bytes())?;
    f.write_all(&(header_size as u32).to_be_bytes())?;

    // write index entries: offset (BE u32), length (BE u32), null-terminated name
    for (i, name) in names.iter().enumerate() {
        f.write_all(&(offsets[i] as u32).to_be_bytes())?;
        f.write_all(&(lengths[i] as u32).to_be_bytes())?;
        f.write_all(name.as_bytes())?;
        f.write_all(&[0u8])?; // null terminator
    }

    // write last header/tail (8 bytes) -- parser ignores this content, keep zeros
    f.write_all(&[0u8; 8])?;

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

/// Append a file into an existing archive by creating a new temporary archive that
/// copies existing entries and inserts the new file. If `force` is false and
/// `archive_target_path` already exists in archive, returns an error.
pub fn append_file_to_archive<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
    archive_path: P,
    source_file: Q,
    archive_target_path: &str,
    force: bool,
) -> anyhow::Result<()> {
    let archive_path = archive_path.as_ref();
    let source_file = source_file.as_ref();

    let (_archive_meta, _index, entries) = crate::parser::parse_archive(archive_path)?;

    // Check collision
    if entries.iter().any(|e| e.name == archive_target_path) && !force {
        return Err(anyhow::anyhow!("target path already exists in archive"));
    }

    // Prepare list of names and lengths (existing entries + new one)
    let mut names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    let mut lengths: Vec<u64> = entries.iter().map(|e| e.length).collect();

    let new_len = std::fs::metadata(source_file)?.len();
    // If exists and force, replace entry by removing old and pushing new name
    if let Some(pos) = names.iter().position(|n| n == archive_target_path) {
        names.remove(pos);
        lengths.remove(pos);
    }
    names.push(archive_target_path.to_string());
    lengths.push(new_len);

    // compute file-headers region and header size like pack_directory
    let mut file_headers_region: u64 = 0;
    for name in &names {
        file_headers_region += 4 + 4 + (name.len() as u64) + 1; // offset + length + name + null
    }

    let header_size: u64 = 16 + file_headers_region + 8; // SBigHeader + headers + tail
    let payload_start = header_size;

    let mut offsets: Vec<u64> = Vec::new();
    let mut cur = payload_start;
    for len in &lengths {
        offsets.push(cur);
        cur += *len;
    }

    // create temp file in same dir
    let mut tmp_path = archive_path.to_path_buf();
    tmp_path.set_extension("bigtmp");

    let mut out = std::fs::File::create(&tmp_path)?;

    // write native BIGF header
    out.write_all(b"BIGF")?;
    out.write_all(&(cur as u32).to_le_bytes())?; // archive size (LE)
    out.write_all(&(names.len() as u32).to_be_bytes())?; // file_count (BE)
    out.write_all(&(header_size as u32).to_be_bytes())?; // header_size (BE)

    // write index entries
    for (i, name) in names.iter().enumerate() {
        out.write_all(&(offsets[i] as u32).to_be_bytes())?;
        out.write_all(&(lengths[i] as u32).to_be_bytes())?;
        out.write_all(name.as_bytes())?;
        out.write_all(&[0u8])?;
    }

    // trailing 8 bytes
    out.write_all(&[0u8; 8])?;

    // copy payloads for existing entries in original archive
    let mut srcf = std::fs::File::open(archive_path)?;
    for e in entries.iter() {
        // if this entry was replaced by new file (force), skip it
        if e.name == archive_target_path {
            continue;
        }
        srcf.seek(std::io::SeekFrom::Start(e.offset))?;
        let mut to_copy = e.length;
        let mut buf = [0u8; 8 * 1024];
        while to_copy > 0 {
            let read = std::cmp::min(buf.len() as u64, to_copy) as usize;
            srcf.read_exact(&mut buf[..read])?;
            out.write_all(&buf[..read])?;
            to_copy -= read as u64;
        }
    }

    // append new file
    let mut newf = std::fs::File::open(source_file)?;
    std::io::copy(&mut newf, &mut out)?;

    out.sync_all()?;

    // replace original
    std::fs::rename(&tmp_path, archive_path)?;

    Ok(())
}

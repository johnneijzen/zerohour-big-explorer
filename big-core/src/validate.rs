use crate::models::{ValidationResult, ValidationIssue};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// Enhanced validation routine:
/// - file exists and readable
/// - header magic matches
/// - index offset/count are within file bounds
/// - index read succeeds and entry count matches header
/// - detect overlapping entries or entries outside file
pub fn validate_archive<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<ValidationResult> {
    let path_ref = path.as_ref();
    let mut result = ValidationResult { errors: vec![], warnings: vec![], issues: vec![] };

    // helper functions to push structured issues
    fn push_error(result: &mut ValidationResult, code: &str, msg: String) {
        result.errors.push(msg.clone());
        result.issues.push(ValidationIssue { code: Some(code.to_string()), message: msg, severity: "error".to_string() });
    }
    fn push_warning(result: &mut ValidationResult, code: &str, msg: String) {
        result.warnings.push(msg.clone());
        result.issues.push(ValidationIssue { code: Some(code.to_string()), message: msg, severity: "warning".to_string() });
    }

    let meta = match std::fs::metadata(path_ref) {
        Ok(m) => m,
        Err(e) => {
            push_error(&mut result, "CANNOT_STAT", format!("Cannot stat file {}: {}", path_ref.display(), e));
            return Ok(result);
        }
    };

    let size = meta.len();

    let mut f = match File::open(path_ref) {
        Ok(ff) => ff,
        Err(e) => {
            push_error(&mut result, "CANNOT_OPEN", format!("Cannot open file {}: {}", path_ref.display(), e));
            return Ok(result);
        }
    };

    // read header (20 bytes expected)
    let mut header = [0u8; 20];
    if let Err(e) = f.read_exact(&mut header) {
        push_error(&mut result, "HEADER_READ_ERROR", format!("Failed reading header for {}: {}", path_ref.display(), e));
        return Ok(result);
    }

    if &header[0..4] != b"BIG\0" {
        push_error(&mut result, "INVALID_MAGIC", format!("Invalid magic for {}", path_ref.display()));
        return Ok(result);
    }

    let version = u32::from_le_bytes(header[4..8].try_into().unwrap_or([0,0,0,0]));
    let index_offset = u64::from_le_bytes(header[8..16].try_into().unwrap_or([0;8]));
    let index_count = u32::from_le_bytes(header[16..20].try_into().unwrap_or([0,0,0,0]));

    if index_offset as u128 >= size as u128 {
        push_error(&mut result, "INDEX_OOB", format!("Index offset {} is outside file (size {})", index_offset, size));
        return Ok(result);
    }

    // Try to read index entries via IndexReader to validate entries parsing
    if let Err(e) = (|| -> anyhow::Result<()> {
        // reuse index reader implementation
        let reader = crate::index::IndexReader::new();
        let entries = reader.read_entries_from(&mut f, index_offset, index_count)?;

        // check that the number of entries matches header
        if entries.len() != index_count as usize {
            push_warning(&mut result, "INDEX_COUNT_MISMATCH", format!("Index count header {} differs from parsed entries {}", index_count, entries.len()));
        }

        // check each entry bounds and build ranges for overlap detection
        let mut ranges: Vec<(u64,u64,String)> = Vec::new();
        for ent in entries.iter() {
            // name sanity checks
            if ent.name.contains("..") || ent.name.starts_with('/') {
                push_warning(&mut result, "PATH_TRAVERSAL", format!("Entry '{}' has suspicious path (possible traversal)", ent.name));
            }

            if ent.length == 0 {
                push_warning(&mut result, "ZERO_LENGTH", format!("Entry '{}' has zero length", ent.name));
            }

            if ent.offset.checked_add(ent.length).map(|v| v > size).unwrap_or(true) {
                push_error(&mut result, "ENTRY_OOB", format!("Entry '{}' (offset {}, len {}) extends past file size {}", ent.name, ent.offset, ent.length, size));
            }
            ranges.push((ent.offset, ent.offset + ent.length, ent.name.clone()));
        }

        // detect duplicate names
        {
            use std::collections::HashMap;
            let mut counts: HashMap<&str, usize> = HashMap::new();
            for ent in entries.iter() {
                *counts.entry(&ent.name).or_default() += 1;
            }
            for (name, cnt) in counts.into_iter() {
                if cnt > 1 {
                    push_warning(&mut result, "DUP_NAME", format!("Duplicate entry name '{}' appears {} times", name, cnt));
                }
            }
        }

        // sort ranges by start and detect overlaps
        ranges.sort_by_key(|r| r.0);
        for w in ranges.windows(2) {
            let a = &w[0];
            let b = &w[1];
            if a.1 > b.0 {
                push_error(&mut result, "ENTRY_OVERLAP", format!("Entries '{}' and '{}' overlap ({}..{} vs {}..{})", a.2, b.2, a.0, a.1, b.0, b.1));
            }
        }

        // ensure payload region begins after index
        if let Ok(cur) = f.seek(SeekFrom::Current(0)) {
            if let Some(min_off) = entries.iter().map(|e| e.offset).min() {
                if min_off < cur {
                    push_error(&mut result, "PAYLOAD_BEFORE_INDEX_END", format!("First payload offset {} is before index end {}", min_off, cur));
                }
            }
        }

        Ok(())
    })() {
        // reading entries failed
        result.errors.push(format!("Failed reading index entries: {}", e));
    }

    // small informational warning about version
    if version == 0 {
        result.warnings.push("Archive version is 0".to_string());
    }

    Ok(result)
}

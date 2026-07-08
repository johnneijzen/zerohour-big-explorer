use crate::models::{ValidationIssue, ValidationResult};
use std::fs::File;
use std::io::{Read, Seek};

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
        result.issues.push(ValidationIssue {
            code: Some(code.to_string()),
            message: msg,
            severity: "error".to_string(),
        });
    }
    fn push_warning(result: &mut ValidationResult, code: &str, msg: String) {
        result.warnings.push(msg.clone());
        result.issues.push(ValidationIssue {
            code: Some(code.to_string()),
            message: msg,
            severity: "warning".to_string(),
        });
    }

    let meta = match std::fs::metadata(path_ref) {
        Ok(m) => m,
        Err(e) => {
            push_error(
                &mut result,
                "CANNOT_STAT",
                format!("Cannot stat file {}: {}", path_ref.display(), e),
            );
            return Ok(result);
        }
    };

    let size = meta.len();

    let mut f = match File::open(path_ref) {
        Ok(ff) => ff,
        Err(e) => {
            push_error(
                &mut result,
                "CANNOT_OPEN",
                format!("Cannot open file {}: {}", path_ref.display(), e),
            );
            return Ok(result);
        }
    };

    // Read first 16 bytes to check header magic and heuristic header fields
    let mut header = [0u8; 16];
    if let Err(e) = f.read_exact(&mut header) {
        push_error(
            &mut result,
            "HEADER_READ_ERROR",
            format!("Failed reading header for {}: {}", path_ref.display(), e),
        );
        return Ok(result);
    }

    if &header[0..4] != b"BIGF" && &header[0..4] != b"BIG4" {
        push_error(
            &mut result,
            "INVALID_MAGIC",
            format!("Invalid magic for {}", path_ref.display()),
        );
        return Ok(result);
    }

    // Try to parse archive using the canonical parser and validate entries
    match crate::parser::parse_archive(path_ref) {
        Ok((_archive, _index, entries)) => {
            // check each entry bounds and build ranges for overlap detection
            let mut ranges: Vec<(u64, u64, String)> = Vec::new();
            for ent in entries.iter() {
                // name sanity checks
                if ent.name.contains("..") || ent.name.starts_with('/') {
                    push_warning(
                        &mut result,
                        "PATH_TRAVERSAL",
                        format!("Entry '{}' has suspicious path (possible traversal)", ent.name),
                    );
                }

                if ent.length == 0 {
                    push_warning(
                        &mut result,
                        "ZERO_LENGTH",
                        format!("Entry '{}' has zero length", ent.name),
                    );
                }

                if ent.offset.checked_add(ent.length).map(|v| v > size).unwrap_or(true) {
                    push_error(
                        &mut result,
                        "ENTRY_OOB",
                        format!(
                            "Entry '{}' (offset {}, len {}) extends past file size {}",
                            ent.name, ent.offset, ent.length, size
                        ),
                    );
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
                        push_warning(
                            &mut result,
                            "DUP_NAME",
                            format!("Duplicate entry name '{}' appears {} times", name, cnt),
                        );
                    }
                }
            }

            // sort ranges by start and detect overlaps
            ranges.sort_by_key(|r| r.0);
            for w in ranges.windows(2) {
                let a = &w[0];
                let b = &w[1];
                if a.1 > b.0 {
                    push_error(
                        &mut result,
                        "ENTRY_OVERLAP",
                        format!(
                            "Entries '{}' and '{}' overlap ({}..{} vs {}..{})",
                            a.2, b.2, a.0, a.1, b.0, b.1
                        ),
                    );
                }
            }
        }
        Err(e) => {
            push_error(&mut result, "PARSE_FAILED", format!("Failed to parse archive: {}", e));
        }
    }

    Ok(result)
}

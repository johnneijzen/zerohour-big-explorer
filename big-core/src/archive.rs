use anyhow::Context;
use std::path::Path;

use crate::models::Archive;

pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Archive> {
    let p = path.as_ref();
    let meta = std::fs::metadata(p).with_context(|| format!("reading metadata for {}", p.display()))?;
    Ok(Archive {
        path: p.to_string_lossy().into_owned(),
        size: meta.len(),
        format_version: None,
    })
}

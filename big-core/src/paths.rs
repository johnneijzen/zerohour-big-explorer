use std::path::{Path, PathBuf};

/// Safely join an archive entry name to an output directory, preventing directory traversal.
pub fn safe_join<P: AsRef<Path>>(output_dir: P, entry_name: &str) -> anyhow::Result<PathBuf> {
    let out = output_dir.as_ref();
    // Normalize entry_name: convert Windows-style backslashes to forward slashes
    // so components are interpreted consistently on Unix systems. Then reject
    // absolute paths and any `..` components to prevent traversal.
    let normalized = entry_name.replace('\\', "/");
    let entry_path = Path::new(&normalized);
    if entry_path.is_absolute() {
        return Err(anyhow::anyhow!("archive entry name is absolute: {}", entry_name));
    }

    for comp in entry_path.components() {
        use std::path::Component;
        if matches!(comp, Component::ParentDir) {
            return Err(anyhow::anyhow!(
                "archive entry name contains parent dir '..': {}",
                entry_name
            ));
        }
    }

    // Join using forward-compatible components
    let mut dest = out.to_path_buf();
    for part in entry_path.iter() {
        dest.push(part);
    }

    Ok(dest)
}

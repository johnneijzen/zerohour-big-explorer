use tauri::command;

#[tauri::command]
pub fn list_archive(
    archive_path: String,
    filter: Option<String>,
) -> Result<Vec<big_core::models::Entry>, String> {
    let (_archive_meta, _index, entries) =
        big_core::parse_archive(&archive_path).map_err(|e| e.to_string())?;

    let filtered = big_core::search::filter_entries(&entries, filter.as_deref());
    Ok(filtered)
}

#[tauri::command]
pub fn extract_entry(
    archive_path: String,
    entry_name: String,
    dest_path: String,
) -> Result<(), String> {
    let (_archive_meta, _index, entries) =
        big_core::parse_archive(&archive_path).map_err(|e| e.to_string())?;

    let entry = entries
        .into_iter()
        .find(|e| e.name == entry_name)
        .ok_or_else(|| format!("entry not found: {}", entry_name))?;

    big_core::extract::extract_entry_to_path(&archive_path, &entry, &dest_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}

use std::path::PathBuf;
use rfd::FileDialog;

#[tauri::command]
pub fn open_dialog() -> Result<Option<String>, String> {
    // Use the plugin dialog's blocking builder to show a native file picker and return the selected path
    match FileDialog::new().add_filter("BIG", &["big"]).pick_file() {
        Some(pathbuf) => Ok(Some(pathbuf.to_string_lossy().to_string())),
        None => Ok(None),
    }
}

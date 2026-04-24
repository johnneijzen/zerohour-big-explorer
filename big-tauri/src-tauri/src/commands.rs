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

use rfd::FileDialog;

#[tauri::command]
pub fn open_dialog() -> Result<Option<String>, String> {
    // Use the plugin dialog's blocking builder to show a native file picker and return the selected path
    match FileDialog::new().add_filter("BIG", &["big"]).pick_file() {
        Some(pathbuf) => Ok(Some(pathbuf.to_string_lossy().to_string())),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn extract_file_bytes(archive_path: String, entry_name: String) -> Result<Vec<u8>, String> {
    let (_archive_meta, _index, entries) =
        big_core::parse_archive(&archive_path).map_err(|e| e.to_string())?;

    let entry = entries
        .into_iter()
        .find(|e| e.name == entry_name)
        .ok_or_else(|| format!("entry not found: {}", entry_name))?;

    big_core::extract::extract_file(&archive_path, &entry).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn extract_file_to_disk(
    archive_path: String,
    entry_name: String,
    output: String,
) -> Result<(), String> {
    let (_archive_meta, _index, entries) =
        big_core::parse_archive(&archive_path).map_err(|e| e.to_string())?;

    let entry = entries
        .into_iter()
        .find(|e| e.name == entry_name)
        .ok_or_else(|| format!("entry not found: {}", entry_name))?;

    big_core::extract::extract_entry_to_path(&archive_path, &entry, &output)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pack_directory(input_dir: String, output_archive: String) -> Result<(), String> {
    big_core::pack::pack_directory(&input_dir, &output_archive).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn append_file(
    archive_path: String,
    source: String,
    target_archive_path: String,
    force: bool,
) -> Result<(), String> {
    big_core::pack::append_file_to_archive(&archive_path, &source, &target_archive_path, force)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn unpack_all(archive_path: String, output_dir: String) -> Result<(), String> {
    big_core::extract::extract_all(&archive_path, &output_dir).map_err(|e| e.to_string())
}

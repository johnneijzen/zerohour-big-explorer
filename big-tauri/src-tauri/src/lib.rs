// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::list_archive,
            commands::extract_entry,
            commands::open_dialog,
            // Added command bindings
            commands::extract_file_bytes,
            commands::extract_file_to_disk,
            commands::pack_directory,
            commands::append_file,
            commands::unpack_all,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

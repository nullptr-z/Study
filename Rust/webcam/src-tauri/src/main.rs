#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_window_decorations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// @4
#[tauri::command]
fn set_window_decorations(window: tauri::Window, decoration: bool) {
    window.set_decorations(decoration).unwrap()
}

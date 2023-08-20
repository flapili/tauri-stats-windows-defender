// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use const_random::const_random;
const RAND_BYTES: [u8; 1024] = const_random!([u8; 1024]);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn test() -> String {
    format!("rand_bytes size: {}", RAND_BYTES.len())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

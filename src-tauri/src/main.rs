// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod file_features {
    pub mod operations;
    pub mod task_control;
    pub mod utils;
}

use tauri::generate_handler;

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            file_features::operations::separate_files_by_extension,
            file_features::operations::undo_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod features;
pub mod shared;

use features::categorize_by_metadata::command::{
    group_by_aperture, group_by_camera_model, group_by_date, group_by_focal_length,
    group_by_gps_location, group_by_iso, group_by_lens, group_by_shutter_speed,
    group_by_time_of_day,
};
use features::list_directory_files::command::{list_directory_files, list_normal_and_raw_files};
use features::organize_files_by_type::command::organize_files_by_type;
use features::organize_raw_files::command::organize_raw_files;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            group_by_aperture,
            group_by_camera_model,
            group_by_date,
            group_by_focal_length,
            group_by_gps_location,
            group_by_iso,
            group_by_lens,
            group_by_shutter_speed,
            group_by_time_of_day,
            list_directory_files,
            list_normal_and_raw_files,
            organize_files_by_type,
            organize_raw_files,
        ])
        .run(tauri::generate_context!())
        .expect("오류 발생: Tauri 애플리케이션 실행 실패");
}

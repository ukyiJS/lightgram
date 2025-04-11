pub mod features;

use features::categorize_by_metadata::command::{
    group_photos_by_aperture, group_photos_by_camera_make, group_photos_by_camera_model,
    group_photos_by_date, group_photos_by_focal_length, group_photos_by_gps_location,
    group_photos_by_iso, group_photos_by_lens, group_photos_by_shutter_speed,
};
use features::convert_raw_to_jpg::command::convert_raw_files_to_jpg;
use features::match_jpg_with_raw::command::get_matched_raw_files;
use features::organize_by_extension::command::organize_files_by_extension;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            organize_files_by_extension,
            convert_raw_files_to_jpg,
            get_matched_raw_files,
            group_photos_by_lens,
            group_photos_by_date,
            group_photos_by_iso,
            group_photos_by_aperture,
            group_photos_by_shutter_speed,
            group_photos_by_focal_length,
            group_photos_by_camera_make,
            group_photos_by_camera_model,
            group_photos_by_gps_location,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

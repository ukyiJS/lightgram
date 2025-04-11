use crate::features::categorize_by_metadata::by_aperture::service::group_by_aperture;
use crate::features::categorize_by_metadata::by_camera_make::service::group_by_camera_make;
use crate::features::categorize_by_metadata::by_camera_model::service::group_by_camera_model;
use crate::features::categorize_by_metadata::by_date::service::group_by_date;
use crate::features::categorize_by_metadata::by_focal_length::service::group_by_focal_length;
use crate::features::categorize_by_metadata::by_gps_location::service::group_by_gps_location;
use crate::features::categorize_by_metadata::by_iso::service::group_by_iso;
use crate::features::categorize_by_metadata::by_lens::service::group_by_lens;
use crate::features::categorize_by_metadata::by_shutter_speed::service::group_by_shutter_speed;

use std::collections::HashMap;
use tauri::command;

#[command]
pub fn group_photos_by_aperture(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    group_by_aperture(&directory)
}

#[command]
pub fn group_photos_by_date(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    group_by_date(&directory)
}

#[command]
pub fn group_photos_by_iso(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    group_by_iso(&directory)
}

#[command]
pub fn group_photos_by_lens(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    group_by_lens(&directory)
}

#[command]
pub fn group_photos_by_shutter_speed(
    directory: String,
) -> Result<HashMap<String, Vec<String>>, String> {
    group_by_shutter_speed(&directory)
}

#[command]
pub fn group_photos_by_focal_length(
    directory: String,
) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    group_by_focal_length(&directory)
}

#[command]
pub fn group_photos_by_camera_model(
    directory: String,
) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    group_by_camera_model(&directory)
}

#[command]
pub fn group_photos_by_camera_make(
    directory: String,
) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    group_by_camera_make(&directory)
}

#[command]
pub fn group_photos_by_gps_location(
    directory: String,
) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    group_by_gps_location(&directory)
}

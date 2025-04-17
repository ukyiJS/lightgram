use crate::features::categorize_by_metadata::by_aperture::service::handle_group_by_aperture;
use crate::features::categorize_by_metadata::by_camera_model::service::handle_group_by_camera_model;
use crate::features::categorize_by_metadata::by_date::service::handle_group_by_date;
use crate::features::categorize_by_metadata::by_focal_length::service::handle_group_by_focal_length;
use crate::features::categorize_by_metadata::by_gps_location::service::handle_group_by_gps_location;
use crate::features::categorize_by_metadata::by_iso::service::handle_group_by_iso;
use crate::features::categorize_by_metadata::by_lens::service::handle_group_by_lens;
use crate::features::categorize_by_metadata::by_shutter_speed::service::handle_group_by_shutter_speed;
use crate::features::categorize_by_metadata::by_time_of_day::service::handle_group_by_time_of_day;

use std::collections::HashMap;
use tauri::command;

#[command]
pub fn group_by_aperture(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    handle_group_by_aperture(&directory)
}

#[command]
pub fn group_by_date(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    handle_group_by_date(&directory)
}

#[command]
pub fn group_by_iso(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    handle_group_by_iso(&directory)
}

#[command]
pub fn group_by_lens(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    handle_group_by_lens(&directory)
}

#[command]
pub fn group_by_shutter_speed(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    handle_group_by_shutter_speed(&directory)
}

#[command]
pub fn group_by_focal_length(
    directory: String,
) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    handle_group_by_focal_length(&directory)
}

#[command]
pub fn group_by_camera_model(
    directory: String,
) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    handle_group_by_camera_model(&directory)
}

#[command]
pub fn group_by_gps_location(
    directory: String,
) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    handle_group_by_gps_location(&directory)
}

#[command]
pub fn group_by_time_of_day(directory: String) -> Result<HashMap<String, Vec<String>>, String> {
    handle_group_by_time_of_day(&directory)
}

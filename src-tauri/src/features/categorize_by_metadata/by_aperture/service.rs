use crate::features::categorize_by_metadata::common;
use exif::{In, Tag};
use std::collections::HashMap;

pub fn handle_group_by_aperture(directory: &str) -> Result<HashMap<String, Vec<String>>, String> {
    common::process_image_files(
        directory,
        |exif| {
            exif.get_field(Tag::FNumber, In::PRIMARY)
                .map(|f| f.display_value().to_string())
        },
        |aperture| format!("F{}", aperture),
    )
}

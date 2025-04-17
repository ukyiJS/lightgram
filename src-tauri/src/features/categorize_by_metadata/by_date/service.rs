use crate::features::categorize_by_metadata::common;
use exif::{In, Tag};
use std::collections::HashMap;

pub fn handle_group_by_date(directory: &str) -> Result<HashMap<String, Vec<String>>, String> {
    common::process_image_files(
        directory,
        |exif| {
            exif.get_field(Tag::DateTimeOriginal, In::PRIMARY).map(|f| {
                let date = f.display_value().to_string();
                date.replace(':', "-")
                    .split_whitespace()
                    .next()
                    .unwrap_or("Unknown")
                    .to_string()
            })
        },
        |date| date.to_string(),
    )
}

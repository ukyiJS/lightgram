use crate::features::categorize_by_metadata::common;
use exif::{In, Tag};
use std::collections::HashMap;

fn extract_gps(exif: &exif::Exif) -> Option<(f64, f64)> {
    let lat = exif.get_field(Tag::GPSLatitude, In::PRIMARY)?;
    let lat_ref = exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY)?;
    let lon = exif.get_field(Tag::GPSLongitude, In::PRIMARY)?;
    let lon_ref = exif.get_field(Tag::GPSLongitudeRef, In::PRIMARY)?;

    let lat_val = if let exif::Value::Rational(ref rats) = lat.value {
        rats.iter()
            .fold(0.0, |acc, r| acc + (r.num as f64 / r.denom as f64))
    } else {
        return None;
    };

    let lon_val = if let exif::Value::Rational(ref rats) = lon.value {
        rats.iter()
            .fold(0.0, |acc, r| acc + (r.num as f64 / r.denom as f64))
    } else {
        return None;
    };

    let lat_signed = if lat_ref.display_value().to_string().contains("S") {
        -lat_val
    } else {
        lat_val
    };
    let lon_signed = if lon_ref.display_value().to_string().contains("W") {
        -lon_val
    } else {
        lon_val
    };

    Some((lat_signed, lon_signed))
}

pub fn handle_group_by_gps_location(
    directory: &str,
) -> Result<HashMap<String, Vec<String>>, String> {
    common::process_image_files(
        directory,
        |exif| extract_gps(&exif).map(|(lat, lon)| format!("{:.4}, {:.4}", lat, lon)),
        |location| location.to_string(),
    )
}

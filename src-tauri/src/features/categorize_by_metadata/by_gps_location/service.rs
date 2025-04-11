use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use exif::{In, Reader, Tag};

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

pub fn group_by_gps_location(directory: &str) -> Result<HashMap<String, Vec<String>>, String> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let dir_path = Path::new(directory);

    if !dir_path.is_dir() {
        return Err("유효한 디렉토리가 아닙니다.".to_string());
    }

    for entry in dir_path.read_dir().map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());
        if let Some(ext) = ext {
            if ext != "jpg" && ext != "jpeg" {
                continue;
            }
        }

        let file = File::open(&file_path).map_err(|e| e.to_string())?;
        let exif = Reader::new()
            .read_from_container(&mut std::io::BufReader::new(&file))
            .map_err(|_| format!("EXIF 읽기 실패: {}", file_path.display()))?;

        let gps_key = extract_gps(&exif)
            .map(|(lat, lon)| format!("{:.4}, {:.4}", lat, lon))
            .unwrap_or_else(|| "Unknown".to_string());

        map.entry(gps_key)
            .or_default()
            .push(file_path.to_string_lossy().to_string());
    }

    Ok(map)
}

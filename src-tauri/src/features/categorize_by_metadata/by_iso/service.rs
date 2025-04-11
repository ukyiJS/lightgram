use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use exif::{Reader, Tag};

pub fn group_by_iso(directory: &str) -> Result<HashMap<String, Vec<String>>, String> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let dir_path = Path::new(directory);
    if !dir_path.is_dir() {
        return Err("유효한 디렉토리가 아닙니다".to_string());
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

        let iso = exif
            .get_field(Tag::PhotographicSensitivity, exif::In::PRIMARY)
            .map(|f| f.display_value().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        map.entry(iso)
            .or_default()
            .push(file_path.to_string_lossy().to_string());
    }

    Ok(map)
}

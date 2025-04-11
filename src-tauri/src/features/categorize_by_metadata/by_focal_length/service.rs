use exif::{In, Reader, Tag};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub fn group_by_focal_length(directory: &str) -> Result<HashMap<String, Vec<String>>, String> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let dir_path = Path::new(directory);

    if !dir_path.is_dir() {
        return Err("유효한 디렉토리가 아닙니다.".to_string());
    }

    for entry in dir_path.read_dir().map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());
        if let Some(ext) = ext {
            if ext != "jpg" && ext != "jpeg" {
                continue;
            }
        }

        let file = File::open(&path).map_err(|e| e.to_string())?;
        let exif = Reader::new()
            .read_from_container(&mut std::io::BufReader::new(&file))
            .map_err(|_| format!("EXIF 데이터 읽기 실패: {}", path.display()))?;

        let focal_length = exif
            .get_field(Tag::FocalLength, In::PRIMARY)
            .map(|f| f.display_value().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        map.entry(focal_length)
            .or_default()
            .push(path.to_string_lossy().to_string());
    }

    Ok(map)
}

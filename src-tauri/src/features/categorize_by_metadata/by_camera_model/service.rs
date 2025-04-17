use crate::shared::file_extensions::is_supported_image;
use exif::{In, Reader, Tag};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

pub fn handle_group_by_camera_model(
    directory: &str,
) -> Result<HashMap<String, Vec<String>>, String> {
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

        // 이미지 파일만 처리
        let is_image_file = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| is_supported_image(ext))
            .unwrap_or(false);

        if !is_image_file {
            continue;
        }

        let file = File::open(&file_path).map_err(|e| e.to_string())?;
        let exif = Reader::new()
            .read_from_container(&mut std::io::BufReader::new(&file))
            .map_err(|_| format!("EXIF 읽기 실패: {}", file_path.display()))?;

        let model = exif
            .get_field(Tag::Model, In::PRIMARY)
            .map(|f| f.display_value().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        map.entry(model)
            .or_default()
            .push(file_path.to_string_lossy().to_string());
    }

    // 파일을 각 카메라 모델 디렉토리로 이동
    for (model, file_paths) in &map {
        let safe_model = model.replace("/", "-").replace("\\", "-").replace(":", "_");
        let model_dir = dir_path.join(safe_model);

        if !model_dir.exists() {
            fs::create_dir(&model_dir).map_err(|e| e.to_string())?;
        }

        for file_path_str in file_paths {
            let file_path = Path::new(file_path_str);
            if let Some(file_name) = file_path.file_name() {
                let target_path = model_dir.join(file_name);
                fs::rename(file_path, &target_path).map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(map)
}

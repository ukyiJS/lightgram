// 이미지 확장자 목록을 관리하는 공통 모듈
use crate::shared::file_extensions::is_supported_image;
use exif::Reader;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

// 메타데이터 추출과 파일 이동을 위한 공통 함수
pub fn process_image_files<F, G>(
    directory: &str,
    extract_metadata: F,
    format_directory_name: G,
) -> Result<HashMap<String, Vec<String>>, String>
where
    F: Fn(&exif::Exif) -> Option<String>,
    G: Fn(&str) -> String,
{
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let dir_path = Path::new(directory);

    if !dir_path.is_dir() {
        return Err("유효한 디렉토리가 아닙니다".to_string());
    }

    // 파일 분류
    for entry in dir_path.read_dir().map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        // 이미지 파일만 처리
        let is_image = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| is_supported_image(ext))
            .unwrap_or(false);

        if !is_image {
            continue;
        }

        let file = File::open(&file_path).map_err(|e| e.to_string())?;
        let exif = Reader::new()
            .read_from_container(&mut std::io::BufReader::new(&file))
            .map_err(|_| format!("EXIF 읽기 실패: {}", file_path.display()))?;

        let metadata = extract_metadata(&exif).unwrap_or_else(|| "Unknown".to_string());

        map.entry(metadata)
            .or_default()
            .push(file_path.to_string_lossy().to_string());
    }

    // 파일 이동
    for (metadata, file_paths) in &map {
        let safe_name = metadata
            .replace("/", "-")
            .replace("\\", "-")
            .replace(":", "_");
        let dir_name = format_directory_name(&safe_name);
        let target_dir = dir_path.join(dir_name);

        if !target_dir.exists() {
            fs::create_dir(&target_dir).map_err(|e| e.to_string())?;
        }

        for file_path_str in file_paths {
            let file_path = Path::new(file_path_str);
            if let Some(file_name) = file_path.file_name() {
                let target_path = target_dir.join(file_name);
                fs::rename(file_path, &target_path).map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(map)
}

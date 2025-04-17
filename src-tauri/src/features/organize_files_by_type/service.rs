use crate::shared::file_extensions::{is_raw_file, is_raw_metadata_file};
use std::fs;
use std::path::Path;

pub fn handle_organize_files_by_type(directory: &str) -> Result<(), String> {
    let dir_path = Path::new(directory);

    if !dir_path.is_dir() {
        return Err(format!("'{}' is not a directory", directory));
    }

    // raw 디렉토리 생성 (RAW 파일용)
    let raw_dir = dir_path.join("raw");
    if !raw_dir.exists() {
        fs::create_dir(&raw_dir).map_err(|e| e.to_string())?;
    }

    let entries = fs::read_dir(dir_path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            let file_name = path.file_name().ok_or("Invalid file name")?;

            // RAW 파일이나 XMP 파일인 경우 raw 디렉토리로 이동
            if is_raw_file(ext) || is_raw_metadata_file(ext) {
                let new_path = raw_dir.join(file_name);
                fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
            } else {
                // 다른 확장자는 기존 처럼 확장자별 디렉토리로 이동
                let target_dir = dir_path.join(&ext_lower);
                if !target_dir.exists() {
                    fs::create_dir(&target_dir).map_err(|e| e.to_string())?;
                }
                let new_path = target_dir.join(file_name);
                fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

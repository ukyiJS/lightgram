use crate::shared::file_extensions::is_raw_file;
use std::fs;
use std::path::Path;

pub fn handle_organize_raw_files(directory: &str) -> Result<(), String> {
    let dir_path = Path::new(directory);

    if !dir_path.is_dir() {
        return Err(format!("'{}' is not a directory", directory));
    }

    // raw 디렉토리 생성
    let raw_dir = dir_path.join("raw");
    if !raw_dir.exists() {
        fs::create_dir(&raw_dir).map_err(|e| e.to_string())?;
    }

    // 디렉토리 내 파일들을 순회하며 RAW 파일 찾기
    let entries = fs::read_dir(dir_path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // 파일 확장자 확인
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            // RAW 파일인 경우 raw 디렉토리로 이동
            if is_raw_file(ext) {
                let file_name = path.file_name().ok_or("Invalid file name")?;
                let new_path = raw_dir.join(file_name);

                fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

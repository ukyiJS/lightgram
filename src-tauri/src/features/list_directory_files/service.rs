use crate::shared::file_extensions::{is_normal_image, is_raw_file};
use std::fs;
use std::path::Path;

use crate::features::list_directory_files::model::FileInfo;

pub fn handle_list_directory_files(directory: &str) -> Result<Vec<FileInfo>, String> {
    let dir_path = Path::new(directory);

    if !dir_path.is_dir() {
        return Err(format!("'{}' is not a directory", directory));
    }

    let entries = fs::read_dir(dir_path).map_err(|e| e.to_string())?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
        let is_file = metadata.is_file();
        let size = metadata.len();

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());

        files.push(FileInfo {
            name,
            path: path.to_string_lossy().to_string(),
            is_file,
            extension,
            size,
        });
    }

    Ok(files)
}

// JPG 및 일반 이미지와 RAW 파일을 각각 분리하여 반환합니다.
pub fn handle_list_normal_and_raw_files(
    directory: &str,
) -> Result<(Vec<FileInfo>, Vec<FileInfo>), String> {
    let all_files = handle_list_directory_files(directory)?;

    // JPG 및 기타 일반 이미지 파일 필터링
    let normal_files = all_files
        .iter()
        .filter(|file| {
            if let Some(ext) = &file.extension {
                file.is_file && is_normal_image(ext)
            } else {
                false
            }
        })
        .cloned()
        .collect();

    // RAW 파일 필터링
    let raw_files = all_files
        .iter()
        .filter(|file| {
            if let Some(ext) = &file.extension {
                file.is_file && is_raw_file(ext)
            } else {
                false
            }
        })
        .cloned()
        .collect();

    Ok((normal_files, raw_files))
}

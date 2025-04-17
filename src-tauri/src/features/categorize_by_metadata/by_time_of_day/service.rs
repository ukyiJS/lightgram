use crate::shared::file_extensions::is_supported_image;
use exif::{In, Reader, Tag};
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::Path;

// 시간대 정의
#[derive(Debug, PartialEq)]
pub enum TimeOfDay {
    Morning,   // 5:00 - 11:59
    Afternoon, // 12:00 - 16:59
    Evening,   // 17:00 - 20:59
    Night,     // 21:00 - 4:59
}

pub fn handle_group_by_time_of_day(
    directory: &str,
) -> Result<HashMap<String, Vec<String>>, String> {
    // 카테고리 초기화
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    result.insert("아침".to_string(), Vec::new());
    result.insert("오후".to_string(), Vec::new());
    result.insert("저녁".to_string(), Vec::new());
    result.insert("밤".to_string(), Vec::new());
    result.insert("알 수 없음".to_string(), Vec::new());

    let dir_path = Path::new(directory);
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(format!("디렉토리가 존재하지 않습니다: {}", directory));
    }

    let entries =
        fs::read_dir(dir_path).map_err(|e| format!("디렉토리를 읽는 중 오류 발생: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("파일 항목을 읽는 중 오류 발생: {}", e))?;
        let path = entry.path();

        // 이미지 파일만 처리
        if path.is_file()
            && path
                .extension()
                .map_or(false, |ext| is_supported_image(ext.to_str().unwrap_or("")))
        {
            let category = get_time_of_day_category(&path)?;

            if let Some(files) = result.get_mut(&category) {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }

    // 파일을 각 시간대 디렉토리로 이동
    for (category, file_paths) in &result {
        // 파일이 있는 카테고리만 처리
        if file_paths.is_empty() {
            continue;
        }

        let category_dir = dir_path.join(category);
        if !category_dir.exists() {
            fs::create_dir(&category_dir).map_err(|e| format!("디렉토리 생성 실패: {}", e))?;
        }

        for file_path_str in file_paths {
            let file_path = Path::new(file_path_str);
            if let Some(file_name) = file_path.file_name() {
                let target_path = category_dir.join(file_name);
                fs::rename(file_path, &target_path)
                    .map_err(|e| format!("파일 이동 실패: {}", e))?;
            }
        }
    }

    Ok(result)
}

fn get_time_of_day_category(file_path: &Path) -> Result<String, String> {
    let file =
        std::fs::File::open(file_path).map_err(|e| format!("파일을 열 수 없습니다: {}", e))?;

    let mut bufreader = BufReader::new(&file);

    // EXIF 데이터 읽기 시도
    match Reader::new().read_from_container(&mut bufreader) {
        Ok(exif) => {
            // 촬영 시간 찾기
            if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                let value = field.display_value().to_string();
                // 시간 추출: 예시 "2023:05:17 14:30:45" -> 14
                let parts: Vec<&str> = value.split(' ').collect();
                if parts.len() >= 2 {
                    let time_parts: Vec<&str> = parts[1].split(':').collect();
                    if time_parts.len() >= 1 {
                        if let Ok(hour) = time_parts[0].parse::<u8>() {
                            return Ok(categorize_by_hour(hour));
                        }
                    }
                }
            }

            // EXIF 데이터가 있지만 시간 정보를 찾을 수 없음
            Ok("알 수 없음".to_string())
        }
        Err(_) => {
            // EXIF 데이터가 없음
            Ok("알 수 없음".to_string())
        }
    }
}

fn categorize_by_hour(hour: u8) -> String {
    match hour {
        5..=11 => "아침".to_string(),  // 5:00 ~ 11:59
        12..=16 => "오후".to_string(), // 12:00 ~ 16:59
        17..=20 => "저녁".to_string(), // 17:00 ~ 20:59
        _ => "밤".to_string(),         // 21:00 ~ 4:59
    }
}

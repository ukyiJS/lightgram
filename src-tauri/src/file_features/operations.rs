use std::time::Instant;

use async_std::fs::rename;
use async_std::path::Path;
use futures::future::join_all;

use crate::file_features::utils::{classify_files_by_extension, is_dir, move_files, MOVE_LOGS};

#[tauri::command]
pub async fn separate_files_by_extension(dir_path: String) -> Result<String, String> {
    let start = Instant::now();
    let path = Path::new(&dir_path);

    if !is_dir(&path).await {
        return Err("현재 경로가 디렉토리가 아닙니다.".to_string());
    }

    let extension_map = classify_files_by_extension(&path).await?;
    let futures = move_files(extension_map, &path).await;
    join_all(futures).await;

    let duration = start.elapsed();
    Ok(format!("{:.2}", duration.as_secs_f64()))
}

#[tauri::command]
pub async fn undo_files() -> Result<bool, String> {
    let mut move_logs = MOVE_LOGS.lock().await;
    while let Some(move_log) = move_logs.pop_back() {
        if let Err(e) = rename(&move_log.new_path, &move_log.original_path).await {
            eprintln!("실행 취소 중 오류가 발생했습니다.: {}", e);
            return Err(e.to_string());
        }
    }

    Ok(true)
}

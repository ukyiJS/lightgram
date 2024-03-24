use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use crate::file_features::task_control::TaskControl;
use async_std::fs::{self as async_fs};
use async_std::path::{Path, PathBuf};
use async_std::sync::Mutex;
use async_std::task;
use futures::stream::StreamExt;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MOVE_LOGS: Mutex<VecDeque<MoveLog>> = Mutex::new(VecDeque::new());
}

#[derive(Clone, Debug)]
pub struct MoveLog {
    pub original_path: PathBuf,
    pub new_path: PathBuf,
}

pub async fn is_dir(path: &Path) -> bool {
    async_fs::metadata(path).await.map_or(false, |m| m.is_dir())
}

pub async fn is_file(path: &Path) -> bool {
    async_fs::metadata(path)
        .await
        .map_or(false, |m| m.is_file())
}

pub async fn move_files(
    extension_map: HashMap<String, Vec<PathBuf>>,
    base_path: &Path,
) -> Vec<task::JoinHandle<()>> {
    let control = Arc::new(Mutex::new(TaskControl::new(10)));

    extension_map
        .into_iter()
        .map(|(ext, paths)| {
            let control_clone = control.clone();
            let ext_dir = base_path.join(&ext);
            paths.into_iter().map(move |path| {
                let control_clone_inner = control_clone.clone();
                let ext_dir_clone = ext_dir.clone();
                task::spawn(async move {
                    TaskControl::acquire(control_clone_inner.clone()).await;
                    if !is_dir(&ext_dir_clone).await {
                        if let Err(e) = async_fs::create_dir_all(&ext_dir_clone).await {
                            eprintln!("Failed to create directory: {}", e);
                            return;
                        }
                    }
                    let new_path = ext_dir_clone.join(path.file_name().unwrap());
                    if let Err(e) = async_fs::rename(&path, &new_path).await {
                        eprintln!("Failed to move file: {}", e);
                    } else {
                        let mut logs = MOVE_LOGS.lock().await;
                        logs.push_back(MoveLog {
                            original_path: path.clone(),
                            new_path: new_path,
                        });
                    }
                    TaskControl::release(control_clone_inner);
                })
            })
        })
        .flatten()
        .collect()
}

pub async fn classify_files_by_extension(
    dir: &Path,
) -> Result<HashMap<String, Vec<PathBuf>>, String> {
    let mut entries = async_fs::read_dir(dir).await.map_err(|e| e.to_string())?;
    let mut extension_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    while let Some(res) = entries.next().await {
        if let Ok(entry) = res {
            let path = entry.path();
            if is_file(&path).await {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    extension_map
                        .entry(ext.to_lowercase())
                        .or_default()
                        .push(path);
                }
            }
        }
    }

    Ok(extension_map)
}

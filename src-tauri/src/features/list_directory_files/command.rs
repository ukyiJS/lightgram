use tauri::command;

use crate::features::list_directory_files::model::{FileInfo, ListDirectoryParams};
use crate::features::list_directory_files::service::{
    handle_list_directory_files, handle_list_normal_and_raw_files,
};

#[command]
pub fn list_directory_files(params: ListDirectoryParams) -> Result<Vec<FileInfo>, String> {
    handle_list_directory_files(&params.directory)
}

#[command]
pub fn list_normal_and_raw_files(
    params: ListDirectoryParams,
) -> Result<(Vec<FileInfo>, Vec<FileInfo>), String> {
    handle_list_normal_and_raw_files(&params.directory)
}

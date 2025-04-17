use tauri::command;

use crate::features::organize_raw_files::model::OrganizeRawParams;
use crate::features::organize_raw_files::service::handle_organize_raw_files;

#[command]
pub fn organize_raw_files(params: OrganizeRawParams) -> Result<(), String> {
    handle_organize_raw_files(&params.directory)
}

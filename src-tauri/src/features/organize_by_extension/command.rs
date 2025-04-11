use tauri::command;

use crate::features::organize_by_extension::model::OrganizeParams;
use crate::features::organize_by_extension::service::organize_by_extension;

#[command]
pub fn organize_files_by_extension(params: OrganizeParams) -> Result<(), String> {
    organize_by_extension(&params.directory)
}

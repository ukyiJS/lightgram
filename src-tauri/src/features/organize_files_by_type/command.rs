use tauri::command;

use crate::features::organize_files_by_type::model::OrganizeParams;
use crate::features::organize_files_by_type::service::handle_organize_files_by_type;

#[command]
pub fn organize_files_by_type(params: OrganizeParams) -> Result<(), String> {
    handle_organize_files_by_type(&params.directory)
}

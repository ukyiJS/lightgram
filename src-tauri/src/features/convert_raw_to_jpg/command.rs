use tauri::command;

use crate::features::convert_raw_to_jpg::model::ConvertParams;
use crate::features::convert_raw_to_jpg::service::convert_raw_to_jpg;

#[command]
pub fn convert_raw_files_to_jpg(params: ConvertParams) -> Result<(), String> {
    convert_raw_to_jpg(&params.input_dir, params.output_dir.as_deref())
}

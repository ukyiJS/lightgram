use crate::features::match_jpg_with_raw::model::{MatchParams, MatchedPair};
use crate::features::match_jpg_with_raw::service::match_jpg_and_raw;
use tauri::command;

#[command]
pub fn get_matched_raw_files(params: MatchParams) -> Result<Vec<MatchedPair>, String> {
    match_jpg_and_raw(&params.jpg_dir, &params.raw_dir)
}

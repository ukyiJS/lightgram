use crate::features::match_jpg_and_raw::model::{MatchParams, MatchedPair};
use crate::features::match_jpg_and_raw::service::handle_match_jpg_and_raw;
use tauri::command;

#[command]
pub fn match_jpg_and_raw(params: MatchParams) -> Result<Vec<MatchedPair>, String> {
    handle_match_jpg_and_raw(&params.jpg_dir, &params.raw_dir)
}

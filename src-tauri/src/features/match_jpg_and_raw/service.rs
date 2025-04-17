use crate::shared::file_extensions::{NORMAL_IMAGE_EXTENSIONS, RAW_EXTENSIONS};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::model::MatchedPair;

fn collect_files(dir: &Path, exts: &[&str]) -> Result<HashMap<String, PathBuf>, String> {
    let mut map = HashMap::new();

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());
        let stem = path.file_stem().and_then(|s| s.to_str());

        if let (Some(ext), Some(stem)) = (ext, stem) {
            if exts.contains(&ext.as_str()) {
                map.insert(stem.to_string(), path);
            }
        }
    }

    Ok(map)
}

pub fn handle_match_jpg_and_raw(jpg_dir: &str, raw_dir: &str) -> Result<Vec<MatchedPair>, String> {
    let jpg_map = collect_files(Path::new(jpg_dir), &NORMAL_IMAGE_EXTENSIONS)?;
    let raw_map = collect_files(Path::new(raw_dir), &RAW_EXTENSIONS)?;

    let mut matched = Vec::new();

    for (stem, jpg_path) in jpg_map {
        if let Some(raw_path) = raw_map.get(&stem) {
            matched.push(MatchedPair {
                jpg_path: jpg_path.to_string_lossy().to_string(),
                raw_path: raw_path.to_string_lossy().to_string(),
            });
        }
    }

    Ok(matched)
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListDirectoryParams {
    pub directory: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_file: bool,
    pub extension: Option<String>,
    pub size: u64,
}

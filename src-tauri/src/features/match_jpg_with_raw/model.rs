use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MatchParams {
    pub jpg_dir: String,
    pub raw_dir: String,
}

#[derive(Serialize)]
pub struct MatchedPair {
    pub jpg_path: String,
    pub raw_path: String,
}

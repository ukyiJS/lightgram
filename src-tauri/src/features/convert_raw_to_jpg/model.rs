use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConvertParams {
    pub input_dir: String,
    pub output_dir: Option<String>,
}

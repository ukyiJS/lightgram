use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OrganizeRawParams {
    pub directory: String,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct OrganizeParams {
    pub directory: String,
}

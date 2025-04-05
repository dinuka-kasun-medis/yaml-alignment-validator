use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FileUpload {
    pub file: String,
}

#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub message: String,
}

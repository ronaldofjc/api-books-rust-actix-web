use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorData {
    pub message: String,
    pub status: u16,
}

impl ErrorData {
    pub fn new(message: String, status: u16) -> Self {
        Self { message, status }
    }
}
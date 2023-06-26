use reqwest::StatusCode;
use serde::{ Deserialize, Serialize };

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub code: u16,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomError {
    pub code: u16,
    pub message: String,
}

impl CustomError {
    pub fn create_status_code<'a>(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
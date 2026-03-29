use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}

pub mod posts;
pub mod users;

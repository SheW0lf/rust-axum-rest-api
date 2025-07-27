use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<String>,
}

pub mod posts;
pub mod users;
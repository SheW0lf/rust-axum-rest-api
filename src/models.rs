use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{ NaiveDateTime};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<String>,
}
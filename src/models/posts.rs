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

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
}

use axum::{extract::{Extension, Path}, Json, http::StatusCode};
use sqlx::PgPool;
use crate::models::{Post, ErrorResponse};

pub async fn get_posts(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Post>>, (StatusCode, Json<ErrorResponse>)> {
    let posts = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse{
            error: "Failed to fetch posts from database".to_string(),
            message: "Failed to fetch posts from database".to_string(),
            details: None,
        })))
        ?;

   match posts.len() {
    0 => Err((StatusCode::NOT_FOUND, Json(ErrorResponse{
        error: "No posts found".to_string(),
        message: "No posts found".to_string(),
        details: None,
    }))),
    _ => Ok(Json(posts)),
   }
}

pub async fn get_post(Extension(pool): Extension<PgPool>, Path(id): Path<i32>) -> Result<Json<Post>, (StatusCode, Json<ErrorResponse>)> {
    let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse{
            error: "Failed to fetch post from database".to_string(),
            message: "Failed to fetch post from database".to_string(),
            details: None,
        })))
        ?;

    match post {
        Some(post) => Ok(Json(post)),
        None => Err((StatusCode::NOT_FOUND, Json(ErrorResponse{
            error: "Post not found".to_string(),
            message: format!("Post with id {} not found", id),
            details: None,
        })))
    }
}

use crate::{
    auth::jwt::AuthUser,
    models::{
        ErrorResponse, SuccessResponse,
        posts::{CreatePost, Post, UpdatePost},
    },
};
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use sqlx::PgPool;

pub async fn get_posts(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Post>>, (StatusCode, Json<ErrorResponse>)> {
    let posts = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch posts from database".to_string(),
                    message: "Failed to fetch posts from database".to_string(),
                    details: None,
                }),
            )
        })?;

    match posts.len() {
        0 => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "No posts found".to_string(),
                message: "No posts found".to_string(),
                details: None,
            }),
        )),
        _ => Ok(Json(posts)),
    }
}

pub async fn get_post(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, (StatusCode, Json<ErrorResponse>)> {
    let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch post from database".to_string(),
                    message: "Failed to fetch post from database".to_string(),
                    details: None,
                }),
            )
        })?;

    match post {
        Some(post) => Ok(Json(post)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Post not found".to_string(),
                message: format!("Post with id {} not found", id),
                details: None,
            }),
        )),
    }
}

async fn fetch_user_posts(
    id: i32,
    pool: &PgPool,
) -> Result<Vec<Post>, (StatusCode, Json<ErrorResponse>)> {
    let posts = sqlx::query_as!(Post, "SELECT * FROM posts WHERE user_id = $1", id)
        .fetch_all(pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch user posts from database".to_string(),
                    message: "Failed to fetch user posts from database".to_string(),
                    details: None,
                }),
            )
        })?;

    match posts.len() {
        0 => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "No posts found".to_string(),
                message: "No posts found".to_string(),
                details: None,
            }),
        )),
        _ => Ok(posts),
    }
}

pub async fn get_user_posts(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Post>>, (StatusCode, Json<ErrorResponse>)> {
    let posts = fetch_user_posts(id, &pool).await?;
    Ok(Json(posts))
}

pub async fn get_current_user_posts(
    auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Post>>, (StatusCode, Json<ErrorResponse>)> {
    let posts = fetch_user_posts(auth_user.user_id, &pool).await?;
    Ok(Json(posts))
}

pub async fn create_post(
    auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
    Json(post): Json<CreatePost>,
) -> Result<Json<Post>, (StatusCode, Json<ErrorResponse>)> {
    let post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (title, body, user_id) VALUES ($1, $2, $3) RETURNING *",
        post.title,
        post.body,
        auth_user.user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create post".to_string(),
                message: "Failed to create post".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(post))
}

pub async fn update_post(
    auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(post): Json<UpdatePost>,
) -> Result<Json<Post>, (StatusCode, Json<ErrorResponse>)> {
    if auth_user.user_id != id {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Failed to update post".to_string(),
                message: "Failed to update post".to_string(),
                details: None,
            }),
        ));
    }

    let post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = COALESCE($1, title), body = COALESCE($2, body) WHERE id = $3 RETURNING *",
        post.title,
        post.body,
        auth_user.user_id
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse{
            error: "Failed to update post".to_string(),
            message: "Failed to update post".to_string(),
            details: None,
        })))
        ?;

    Ok(Json(post))
}

pub async fn delete_post(
    auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    if auth_user.user_id != id {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Failed to delete post".to_string(),
                message: "Failed to delete post".to_string(),
                details: None,
            }),
        ));
    }

    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", auth_user.user_id)
        .execute(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to delete post".to_string(),
                    message: "Failed to delete post".to_string(),
                    details: None,
                }),
            )
        })?;

    match result.rows_affected() {
        0 => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Post not found".to_string(),
                message: format!("Post with id {} not found", id),
                details: None,
            }),
        )),
        _ => Ok(Json(SuccessResponse {
            message: format!("Post with id {} successfully deleted", id),
        })),
    }
}

use crate::models::{
    ErrorResponse,
    users::{CreateUser, UpdateUser, User},
};
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use sqlx::PgPool;

pub async fn get_users(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<ErrorResponse>)> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch users from database".to_string(),
                    message: "Failed to fetch users from database".to_string(),
                    details: None,
                }),
            )
        })?;

    match users.len() {
        0 => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "No users found".to_string(),
                message: "No users found".to_string(),
                details: None,
            }),
        )),
        _ => Ok(Json(users)),
    }
}

pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch user from database".to_string(),
                    message: "Failed to fetch user from database".to_string(),
                    details: None,
                }),
            )
        })?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User not found".to_string(),
                message: format!("User with id {} not found", id),
                details: None,
            }),
        )),
    }
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *",
        user.username,
        user.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create user".to_string(),
                message: "Failed to create user".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(user))
}

pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(User, "UPDATE users SET username = COALESCE($1, username), email = COALESCE($2, email) WHERE id = $3 RETURNING *", user.username, user.email, id)
        .fetch_one(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse{
            error: "Failed to update user".to_string(),
            message: "Failed to update user".to_string(),
            details: None,
        })))
        ?;

    Ok(Json(user))
}

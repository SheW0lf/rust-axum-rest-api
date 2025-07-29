use crate::{
    auth::jwt::{AuthUser, generate_token},
    models::{
        ErrorResponse, SuccessResponse,
        users::{CreateUser, LoginRequest, LoginResponse, UpdateUser, User, UserSafe},
    },
};
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use sqlx::PgPool;

pub async fn get_users(
    _auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<UserSafe>>, (StatusCode, Json<ErrorResponse>)> {
    let users = sqlx::query_as!(
        UserSafe,
        "SELECT id, username, email, created_at FROM users"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
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
    _auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<UserSafe>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(
        UserSafe,
        "SELECT id, username, email, created_at FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
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
                error: "Unauthorized".to_string(),
                message: format!("User with id {} not found", id),
                details: None,
            }),
        )),
    }
}

pub async fn get_current_user(
    auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<UserSafe>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(
        UserSafe,
        "SELECT id, username, email, created_at FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
                message: "Failed to fetch user from database".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(user.unwrap()))
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<CreateUser>,
) -> Result<Json<UserSafe>, (StatusCode, Json<ErrorResponse>)> {
    let password_hash = User::hash_password(&user.password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
                message: "Failed to hash password".to_string(),
                details: None,
            }),
        )
    })?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        user.username,
        user.email,
        password_hash
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
                message: "Failed to create user".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(user.into()))
}

pub async fn update_user(
    auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<UserSafe>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(UserSafe, "UPDATE users SET username = COALESCE($1, username), email = COALESCE($2, email) WHERE id = $3 RETURNING id, username, email, created_at", user.username, user.email, auth_user.user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse{
            error: e.to_string(),
            message: "Failed to update user".to_string(),
            details: None,
        })))
        ?;

    Ok(Json(user))
}

pub async fn delete_user(
    auth_user: AuthUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", auth_user.user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                    message: "Failed to delete user".to_string(),
                    details: None,
                }),
            )
        })?;

    match result.rows_affected() {
        0 => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: format!("User with id {} not found", auth_user.user_id),
                details: None,
            }),
        )),
        _ => Ok(Json(SuccessResponse {
            message: format!("User with id {} successfully deleted", auth_user.user_id),
        })),
    }
}

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        login_request.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
                message: "Database error".to_string(),
                details: None,
            }),
        )
    })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid credentials".to_string(),
                message: "Username or password is incorrect".to_string(),
                details: None,
            }),
        )
    })?;

    if !user.verify_password(&login_request.password) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid credentials".to_string(),
                message: "Username or password is incorrect".to_string(),
                details: None,
            }),
        ));
    }

    let token = generate_token(user.id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.error,
                message: e.message,
                details: e.details,
            }),
        )
    })?;

    Ok(Json(LoginResponse {
        token,
        user: user.into(),
    }))
}

pub async fn logout(
    _auth_user: AuthUser,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Since JWT is stateless, we just return success
    // In a real app, you might maintain a blacklist of tokens
    Ok(Json(SuccessResponse {
        message: "Successfully logged out".to_string(),
    }))
}

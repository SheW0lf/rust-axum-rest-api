use crate::{
    auth::jwt::{AuthUser, generate_refresh_token, generate_token, hash_token},
    models::{
        ErrorResponse, SuccessResponse,
        users::{
            CreateUser, LoginRequest, LoginResponse, RefreshRequest, RefreshResponse, UpdateUser,
            User, UserSafe,
        },
    },
};
use axum::{
    Json,
    extract::{Extension, Path},
    http::{HeaderMap, StatusCode},
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

    Ok(Json(users))
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
                error: "User not found".to_string(),
                message: format!("User with id {id} not found"),
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

    let user = user.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User not found".to_string(),
                message: "Authenticated user not found".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(user))
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

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let access_token = generate_token(user.id, &jwt_secret).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.error,
                message: e.message,
                details: e.details,
            }),
        )
    })?;

    let (refresh_plaintext, refresh_hash) = generate_refresh_token();
    let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::days(7);

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user.id,
        refresh_hash,
        expires_at,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
                message: "Failed to store refresh token".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token: refresh_plaintext,
        user: user.into(),
    }))
}

pub async fn refresh(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, (StatusCode, Json<ErrorResponse>)> {
    let token_hash = hash_token(&body.refresh_token);
    let now = chrono::Utc::now().naive_utc();

    let record = sqlx::query!(
        "SELECT id, user_id FROM refresh_tokens WHERE token_hash = $1 AND expires_at > $2",
        token_hash,
        now,
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
    })?
    .ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid or expired refresh token".to_string(),
                message: "Please log in again".to_string(),
                details: None,
            }),
        )
    })?;

    sqlx::query!("DELETE FROM refresh_tokens WHERE id = $1", record.id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                    message: "Failed to rotate refresh token".to_string(),
                    details: None,
                }),
            )
        })?;

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let access_token = generate_token(record.user_id, &jwt_secret).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.error,
                message: e.message,
                details: e.details,
            }),
        )
    })?;

    let (refresh_plaintext, refresh_hash) = generate_refresh_token();
    let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::days(7);

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        record.user_id,
        refresh_hash,
        expires_at,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
                message: "Failed to store refresh token".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(RefreshResponse {
        access_token,
        refresh_token: refresh_plaintext,
    }))
}

pub async fn logout(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let refresh_token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Missing or invalid Authorization header".to_string(),
                message: "Authorization header with Bearer token required".to_string(),
                details: None,
            }),
        ))?;

    let token_hash = hash_token(refresh_token);

    sqlx::query!(
        "DELETE FROM refresh_tokens WHERE token_hash = $1",
        token_hash,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
                message: "Failed to revoke refresh token".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(SuccessResponse {
        message: "Successfully logged out".to_string(),
    }))
}

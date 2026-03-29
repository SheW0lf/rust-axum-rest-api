pub mod auth;
pub mod handlers;
pub mod models;
pub mod routes;

use axum::{Extension, Router, routing::get};
use sqlx::PgPool;

async fn root(Extension(pool): Extension<PgPool>) -> impl axum::response::IntoResponse {
    use axum::{Json, http::StatusCode};
    use serde_json::json;

    let db_status = match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "database": db_status,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "service": "rust-axum-rest-api"
        })),
    )
}

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .merge(routes::posts::posts_routes())
        .merge(routes::users::users_routes())
        .layer(Extension(pool))
}

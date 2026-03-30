pub mod auth;
pub mod handlers;
pub mod models;
pub mod routes;

use axum::{Extension, Router, http::HeaderValue, routing::get};
use sqlx::PgPool;
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};

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
    let origin = std::env::var("FRONTEND_ORIGIN")
        .expect("FRONTEND_ORIGIN must be set")
        .parse::<HeaderValue>()
        .expect("FRONTEND_ORIGIN is not a valid header value");

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any());

    Router::new()
        .route("/", get(root))
        .merge(routes::posts::posts_routes())
        .merge(routes::users::users_routes())
        .layer(cors)
        .layer(Extension(pool))
}

mod auth;
mod handlers;
mod models;
mod routes;

use axum::{Extension, Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use dotenvy::dotenv;
use serde_json::json;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::{Level, info};
use tracing_subscriber;

async fn root(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    info!("Starting the server");

    dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    info!("Connected to the database");

    let app: Router = Router::new()
        .route("/", get(root))
        .merge(routes::posts::posts_routes())
        .merge(routes::users::users_routes())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Listening on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

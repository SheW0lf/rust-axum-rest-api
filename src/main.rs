use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::{routing::get, Router, http::StatusCode, response::IntoResponse, Extension};
use tracing::{info, Level};
use tracing_subscriber;

async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    info!("Starting the server");

    dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new().connect(&database_url).await.expect("Failed to connect to the database");
    info!("Connected to the database");
    
    let app: Router = Router::new().route("/", get(root)).layer(Extension(pool));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Listening on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

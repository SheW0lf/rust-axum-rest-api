use dotenvy::dotenv;
use rust_axum_rest_api::create_app;
use sqlx::postgres::PgPoolOptions;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    info!("Starting the server");

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    info!("Connected to the database");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Listening on http://0.0.0.0:5000");
    axum::serve(listener, create_app(pool)).await.unwrap();

    Ok(())
}

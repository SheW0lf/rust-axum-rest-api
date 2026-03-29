use axum_test::TestServer;
use rust_axum_rest_api::create_app;
use sqlx::{PgPool, Row};

pub const TEST_JWT_SECRET: &str = "test-secret";

pub fn token_for(user_id: i32) -> String {
    rust_axum_rest_api::auth::jwt::generate_token(user_id, TEST_JWT_SECRET).unwrap()
}

pub fn bearer(user_id: i32) -> String {
    format!("Bearer {}", token_for(user_id))
}

pub fn server(pool: PgPool) -> TestServer {
    TestServer::new(create_app(pool)).unwrap()
}

pub async fn insert_test_user(pool: &PgPool) -> i32 {
    sqlx::query(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind("testuser")
    .bind("test@example.com")
    .bind("irrelevant-hash")
    .fetch_one(pool)
    .await
    .unwrap()
    .get::<i32, _>("id")
}

mod common;

use serde_json::{Value, json};
use sqlx::PgPool;

#[sqlx::test(migrations = "./migrations")]
async fn create_user_returns_200(pool: PgPool) {
    let server = common::server(pool);

    let res = server
        .post("/user")
        .json(&json!({
            "username": "alice",
            "email": "alice@example.com",
            "password": "password123"
        }))
        .await;

    assert_eq!(res.status_code(), 200);
    let body: Value = res.json();
    assert_eq!(body["username"], "alice");
    assert_eq!(body["email"], "alice@example.com");
    assert!(body.get("password_hash").is_none(), "password_hash must not be serialized");
}

#[sqlx::test(migrations = "./migrations")]
async fn login_success_returns_tokens(pool: PgPool) {
    let server = common::server(pool);

    server
        .post("/user")
        .json(&json!({
            "username": "alice",
            "email": "alice@example.com",
            "password": "password123"
        }))
        .await;

    let res = server
        .post("/auth/login")
        .json(&json!({ "username": "alice", "password": "password123" }))
        .await;

    assert_eq!(res.status_code(), 200);
    let body: Value = res.json();
    assert!(body["access_token"].as_str().is_some());
    assert!(body["refresh_token"].as_str().is_some());
    assert_eq!(body["user"]["username"], "alice");
}

#[sqlx::test(migrations = "./migrations")]
async fn login_wrong_password_returns_401(pool: PgPool) {
    let server = common::server(pool);

    server
        .post("/user")
        .json(&json!({
            "username": "alice",
            "email": "alice@example.com",
            "password": "password123"
        }))
        .await;

    let res = server
        .post("/auth/login")
        .json(&json!({ "username": "alice", "password": "wrong" }))
        .await;

    assert_eq!(res.status_code(), 401);
}

#[sqlx::test(migrations = "./migrations")]
async fn login_unknown_user_returns_401(pool: PgPool) {
    let server = common::server(pool);

    let res = server
        .post("/auth/login")
        .json(&json!({ "username": "nobody", "password": "x" }))
        .await;

    assert_eq!(res.status_code(), 401);
}

#[sqlx::test(migrations = "./migrations")]
async fn get_current_user_requires_auth(pool: PgPool) {
    let server = common::server(pool);
    let res = server.get("/user").await;
    assert_eq!(res.status_code(), 401);
}

#[sqlx::test(migrations = "./migrations")]
async fn get_current_user_returns_200(pool: PgPool) {
    let server = common::server(pool.clone());
    let user_id = common::insert_test_user(&pool).await;

    let res = server
        .get("/user")
        .add_header("Authorization", common::bearer(user_id))
        .await;

    assert_eq!(res.status_code(), 200);
    let body: Value = res.json();
    assert_eq!(body["id"], user_id);
}

#[sqlx::test(migrations = "./migrations")]
async fn refresh_token_returns_new_tokens(pool: PgPool) {
    let server = common::server(pool);

    server
        .post("/user")
        .json(&json!({
            "username": "alice",
            "email": "alice@example.com",
            "password": "password123"
        }))
        .await;

    let login: Value = server
        .post("/auth/login")
        .json(&json!({ "username": "alice", "password": "password123" }))
        .await
        .json();

    let refresh_token = login["refresh_token"].as_str().unwrap();

    let res = server
        .post("/auth/refresh")
        .json(&json!({ "refresh_token": refresh_token }))
        .await;

    assert_eq!(res.status_code(), 200);
    let body: Value = res.json();
    assert!(body["access_token"].as_str().is_some());
    assert!(body["refresh_token"].as_str().is_some());
    assert_ne!(body["refresh_token"].as_str(), Some(refresh_token), "token should be rotated");
}

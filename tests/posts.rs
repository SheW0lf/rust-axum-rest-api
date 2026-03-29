mod common;

use serde_json::{Value, json};
use sqlx::{PgPool, Row};

#[sqlx::test(migrations = "./migrations")]
async fn get_posts_requires_auth(pool: PgPool) {
    let server = common::server(pool);
    let res = server.get("/posts").await;
    assert_eq!(res.status_code(), 401);
}

#[sqlx::test(migrations = "./migrations")]
async fn get_posts_empty_returns_404(pool: PgPool) {
    let server = common::server(pool.clone());
    let user_id = common::insert_test_user(&pool).await;

    let res = server
        .get("/posts")
        .add_header("Authorization", common::bearer(user_id))
        .await;

    assert_eq!(res.status_code(), 404);
}

#[sqlx::test(migrations = "./migrations")]
async fn create_post_returns_200(pool: PgPool) {
    let server = common::server(pool.clone());
    let user_id = common::insert_test_user(&pool).await;

    let res = server
        .post("/post")
        .add_header("Authorization", common::bearer(user_id))
        .json(&json!({ "title": "Hello", "body": "World" }))
        .await;

    assert_eq!(res.status_code(), 200);
    let body: Value = res.json();
    assert_eq!(body["title"], "Hello");
    assert_eq!(body["body"], "World");
    assert_eq!(body["user_id"], user_id);
}

#[sqlx::test(migrations = "./migrations")]
async fn get_post_returns_200(pool: PgPool) {
    let server = common::server(pool.clone());
    let user_id = common::insert_test_user(&pool).await;

    let created: Value = server
        .post("/post")
        .add_header("Authorization", common::bearer(user_id))
        .json(&json!({ "title": "Hello", "body": "World" }))
        .await
        .json();

    let id = created["id"].as_i64().unwrap();

    let res = server
        .get(&format!("/post/{id}"))
        .add_header("Authorization", common::bearer(user_id))
        .await;

    assert_eq!(res.status_code(), 200);
    let body: Value = res.json();
    assert_eq!(body["id"], id);
}

#[sqlx::test(migrations = "./migrations")]
async fn get_post_not_found_returns_404(pool: PgPool) {
    let server = common::server(pool.clone());
    let user_id = common::insert_test_user(&pool).await;

    let res = server
        .get("/post/99999")
        .add_header("Authorization", common::bearer(user_id))
        .await;

    assert_eq!(res.status_code(), 404);
}

#[sqlx::test(migrations = "./migrations")]
async fn update_post_returns_200(pool: PgPool) {
    let server = common::server(pool.clone());
    let user_id = common::insert_test_user(&pool).await;

    let created: Value = server
        .post("/post")
        .add_header("Authorization", common::bearer(user_id))
        .json(&json!({ "title": "Original", "body": "Body" }))
        .await
        .json();

    let id = created["id"].as_i64().unwrap();

    let res = server
        .put(&format!("/post/{id}"))
        .add_header("Authorization", common::bearer(user_id))
        .json(&json!({ "title": "Updated" }))
        .await;

    assert_eq!(res.status_code(), 200);
    let body: Value = res.json();
    assert_eq!(body["title"], "Updated");
    assert_eq!(body["body"], "Body");
}

#[sqlx::test(migrations = "./migrations")]
async fn update_post_wrong_user_returns_404(pool: PgPool) {
    let server = common::server(pool.clone());
    let owner_id = common::insert_test_user(&pool).await;
    let other_id: i32 = sqlx::query(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind("otheruser")
    .bind("other@example.com")
    .bind("hash")
    .fetch_one(&pool)
    .await
    .unwrap()
    .get::<i32, _>("id");

    let created: Value = server
        .post("/post")
        .add_header("Authorization", common::bearer(owner_id))
        .json(&json!({ "title": "Mine", "body": "Body" }))
        .await
        .json();

    let id = created["id"].as_i64().unwrap();

    let res = server
        .put(&format!("/post/{id}"))
        .add_header("Authorization", common::bearer(other_id))
        .json(&json!({ "title": "Stolen" }))
        .await;

    assert_eq!(res.status_code(), 404);
}

#[sqlx::test(migrations = "./migrations")]
async fn delete_post_returns_200(pool: PgPool) {
    let server = common::server(pool.clone());
    let user_id = common::insert_test_user(&pool).await;

    let created: Value = server
        .post("/post")
        .add_header("Authorization", common::bearer(user_id))
        .json(&json!({ "title": "Bye", "body": "Body" }))
        .await
        .json();

    let id = created["id"].as_i64().unwrap();

    let res = server
        .delete(&format!("/post/{id}"))
        .add_header("Authorization", common::bearer(user_id))
        .await;

    assert_eq!(res.status_code(), 200);

    let fetch = server
        .get(&format!("/post/{id}"))
        .add_header("Authorization", common::bearer(user_id))
        .await;
    assert_eq!(fetch.status_code(), 404);
}

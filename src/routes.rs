use axum::{routing::{get, post, put}, Router};
use crate::handlers::{get_posts, get_post, create_post, update_post};

pub fn posts_routes() -> Router {
    Router::new()
    .route("/posts", get(get_posts))
    .route("/posts/{id}", get(get_post))
    .route("/posts", post(create_post))
    .route("/posts/{id}", put(update_post))
}   
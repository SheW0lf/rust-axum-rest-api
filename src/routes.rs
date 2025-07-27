use axum::{routing::{get}, Router};
use crate::handlers::{get_posts, get_post};

pub fn posts_routes() -> Router {
    Router::new()
    .route("/posts", get(get_posts))
    .route("/posts/{id}", get(get_post))
}   
use crate::handlers::posts::{create_post, get_post, get_posts, update_post};
use axum::{
    Router,
    routing::{get, post, put},
};

pub fn posts_routes() -> Router {
    Router::new()
        .route("/posts", get(get_posts))
        .route("/posts/{id}", get(get_post))
        .route("/posts", post(create_post))
        .route("/posts/{id}", put(update_post))
}

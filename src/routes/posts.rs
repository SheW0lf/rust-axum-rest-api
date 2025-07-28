use crate::handlers::posts::{
    create_post, delete_post, get_current_user_posts, get_post, get_posts, get_user_posts,
    update_post,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn posts_routes() -> Router {
    Router::new()
        .route("/posts", get(get_posts))
        .route("/post/{id}", get(get_post))
        .route("/post", post(create_post))
        .route("/post/{id}", put(update_post))
        .route("/post/{id}", delete(delete_post))
        .route("/user/{id}/posts", get(get_user_posts))
        .route("/user/posts", get(get_current_user_posts))
}

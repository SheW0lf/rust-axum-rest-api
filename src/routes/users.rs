use crate::handlers::users::{create_user, get_user, get_users, update_user};
use axum::{
    Router,
    routing::{get, post, put},
};

pub fn users_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user))
        .route("/users/{id}", put(update_user))
}

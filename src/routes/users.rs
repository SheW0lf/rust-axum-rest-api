use crate::handlers::users::{
    create_user, delete_user, get_current_user, get_user, get_users, login, logout, update_user,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn users_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user))
        .route("/user", post(create_user))
        .route("/user", put(update_user))
        .route("/user", delete(delete_user))
        .route("/user", get(get_current_user))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
}

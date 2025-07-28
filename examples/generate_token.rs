use dotenvy;
use rust_axum_rest_api::auth::jwt::generate_token;

fn main() {
    dotenvy::dotenv().ok();

    let user_id = std::env::args()
        .nth(1)
        .and_then(|id| id.parse::<i32>().ok())
        .unwrap_or(1);

    match generate_token(user_id) {
        Ok(token) => println!("{}", token),
        Err(e) => println!("Error: {}", e.error),
    }
}

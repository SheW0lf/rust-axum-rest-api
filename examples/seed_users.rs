use dotenvy;
use rust_axum_rest_api::models::users::User;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    let users = vec![
        ("john_doe", "john@example.com", "password123"),
        ("jane_smith", "jane@example.com", "password123"),
        ("admin", "admin@example.com", "admin_secure"),
    ];

    for (username, email, password) in users {
        let password_hash = User::hash_password(password)?;

        sqlx::query!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) ON CONFLICT (username) DO NOTHING",
            username,
            email,
            password_hash
        )
        .execute(&pool)
        .await?;

        println!("✅ Seeded user: {} (password: {})", username, password);
    }

    println!("🌱 User seeding complete!");
    Ok(())
}

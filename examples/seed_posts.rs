use dotenvy;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    let posts = vec![
        ("First Post", "This is my first post content.", "john_doe"),
        ("Hello World", "Welcome to our platform!", "john_doe"),
        ("Getting Started", "A guide for new users.", "jane_smith"),
        (
            "Admin Announcement",
            "Important updates coming soon.",
            "admin",
        ),
    ];

    for (title, body, username) in posts {
        // Get user_id by username
        let user_id = sqlx::query_scalar!("SELECT id FROM users WHERE username = $1", username)
            .fetch_optional(&pool)
            .await?;

        match user_id {
            Some(user_id) => {
                sqlx::query!(
                    "INSERT INTO posts (title, body, user_id) VALUES ($1, $2, $3)",
                    title,
                    body,
                    user_id
                )
                .execute(&pool)
                .await?;

                println!("✅ Seeded post: '{}' by {}", title, username);
            }
            None => {
                println!(
                    "⚠️  Skipped post '{}' - user '{}' not found",
                    title, username
                );
            }
        }
    }

    println!("🌱 Posts seeding complete!");
    Ok(())
}

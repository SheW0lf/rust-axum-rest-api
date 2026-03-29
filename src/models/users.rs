use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserSafe,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserSafe {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl From<User> for UserSafe {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

impl User {
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        if let Some(ref hash) = self.password_hash {
            bcrypt::verify(password, hash).unwrap_or(false)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_user(password_hash: Option<String>) -> User {
        User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            password_hash,
        }
    }

    #[test]
    fn hash_password_produces_verifiable_hash() {
        let hash = User::hash_password("secret").unwrap();
        assert!(bcrypt::verify("secret", &hash).unwrap());
    }

    #[test]
    fn hash_password_is_not_plaintext() {
        let hash = User::hash_password("secret").unwrap();
        assert_ne!(hash, "secret");
    }

    #[test]
    fn verify_password_correct() {
        let hash = User::hash_password("hunter2").unwrap();
        let user = make_user(Some(hash));
        assert!(user.verify_password("hunter2"));
    }

    #[test]
    fn verify_password_wrong() {
        let hash = User::hash_password("hunter2").unwrap();
        let user = make_user(Some(hash));
        assert!(!user.verify_password("wrong"));
    }

    #[test]
    fn verify_password_no_hash() {
        let user = make_user(None);
        assert!(!user.verify_password("anything"));
    }

    #[test]
    fn user_safe_from_user() {
        let user = make_user(Some("hash".to_string()));
        let safe = UserSafe::from(user);
        assert_eq!(safe.id, 1);
        assert_eq!(safe.username, "testuser");
        assert_eq!(safe.email, "test@example.com");
    }
}

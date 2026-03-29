use crate::{auth::claims::Claims, models::ErrorResponse};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: i32,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token_data = decode::<Claims>(
            auth_header,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
        })
    }
}

pub fn generate_token(user_id: i32) -> Result<String, ErrorResponse> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: user_id,
        exp: now + 900, // 15 minutes
        iat: now,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| ErrorResponse {
        error: e.to_string(),
        message: "Failed to generate token".to_string(),
        details: None,
    })
}

pub fn generate_refresh_token() -> (String, String) {
    let mut bytes = [0u8; 32]; // array of 32 0's (unsigned 8-bit integers for memory). 
    rand::thread_rng().fill_bytes(&mut bytes); //fill_bytes fills the bytes array with random u8 bytes (0 - 255) but we have to initialize it first and then overwrite it because Rust requires memory to be initialized before use.
    let plaintext: String = bytes.iter().map(|b| format!("{b:02x}")).collect(); // formats each byte as a 2-character hexadecimal (02 for char and x for hex) string and at the end combines (collect) them into a single string.
    let hash = hash_token(&plaintext); // hashes our hex string into a 64-character hexadecimal string.
    (plaintext, hash) // returns the plaintext and hash as a tuple.
}

pub fn hash_token(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}

use crate::{auth::claims::Claims, models::ErrorResponse};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: i32,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
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
}

#[allow(dead_code)] // TODO: Remove this once we implement login functionality
pub fn generate_token(user_id: i32) -> Result<String, ErrorResponse> {
    use jsonwebtoken::{EncodingKey, Header, encode};
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: user_id,
        exp: now + 3600,
        iat: now,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| ErrorResponse {
        error: "Failed to generate token".to_string(),
        message: e.to_string(),
        details: None,
    })
}

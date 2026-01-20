use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use crate::api::ApiError;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (usually user_id or username)
    pub exp: usize,  // Expiration time (as a timestamp)
    pub iat: usize,  // Issued at
}

pub struct AuthUser {
    pub user_id: String,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = crate::api::ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let state = parts
            .extensions
            .get::<crate::AppState>()
            .ok_or_else(|| ApiError::Internal("AppState not found in request extensions".into()))?;

        // 1. Get the Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".into()))?;

        // 2. Check if it starts with "Bearer "
        if !auth_header.starts_with("Bearer ") {
            return Err(ApiError::Unauthorized("Invalid token type".into()));
        }

        let token = &auth_header[7..]; // Strip "Bearer "

        // 3. Decode and validate
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| ApiError::Unauthorized(format!("Invalid or expired token: {}", e)))?;

        // 4. Return the authenticated user
        Ok(AuthUser {
            user_id: token_data.claims.sub,
        })
    }
}

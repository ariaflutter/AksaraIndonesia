// src/auth/handlers.rs

use axum::{extract::Extension, http::StatusCode, Json};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::users::model::User; // We need the full User model to get the password hash
use super::model::{Claims, LoginRequest, LoginResponse};

// A secret key for signing the JWT.
// IMPORTANT: In a real application, this MUST be loaded from the .env file
// and should be a long, randomly generated string.
const JWT_SECRET: &[u8] = b"your-super-secret-and-long-key";

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    
    // 1. Find the user by NIP in the database.
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE nip = $1")
        .bind(&payload.nip)
        .fetch_optional(&pool) // Use `fetch_optional` as the user might not exist.
        .await
        .map_err(|e| {
            tracing::error!("Database error during login: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?; // If no user is found, return Unauthorized.

    // 2. Verify the submitted password against the stored hash.
    let password_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| {
            // This error means the hash is invalid, which is a server problem.
            tracing::error!("Invalid password hash for user {}", user.id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !password_valid {
        // If the password does not match, return Unauthorized.
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // 3. If password is valid, create the JWT claims.
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let claims = Claims {
        sub: user.id,
        role: user.role,
        unit_kerja_id: user.unit_kerja_id,
        // Token expires in 1 day.
        exp: (now + 60 * 60 * 24) as usize,
    };
    
    // 4. Encode the claims into a JWT.
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|e| {
            tracing::error!("Failed to create JWT: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 5. Send the token back to the user.
    let response = LoginResponse { token };
    Ok(Json(response))
}
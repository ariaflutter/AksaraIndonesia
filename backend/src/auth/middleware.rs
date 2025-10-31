// src/auth/middleware.rs

use axum::{
    body::Body, // <-- Import the concrete Body type
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::auth::model::{AuthenticatedUser, Claims}; 

const JWT_SECRET: &[u8] = b"your-super-secret-and-long-key";

// The function signature no longer needs to be generic over the body type `B`.
// We now explicitly work with `Request<Body>`.
pub async fn auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    
    let token = req.headers()
        .get("authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let token = token.ok_or_else(|| {
        tracing::warn!("Request is missing bearer token");
        StatusCode::UNAUTHORIZED
    })?;

    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|e| {
        tracing::warn!("Token validation failed: {}", e);
        StatusCode::UNAUTHORIZED
    })?
    .claims;
       
    let authenticated_user = AuthenticatedUser {
        id: claims.sub,
        role: claims.role,
        bapas_id: claims.bapas_id, // Mapping dari token ke struct internal
        kanwil_id: claims.kanwil_id,
    };
    req.extensions_mut().insert(authenticated_user);


    // Now, `req` is guaranteed to be the correct type that `next.run()` expects.
    Ok(next.run(req).await)
}
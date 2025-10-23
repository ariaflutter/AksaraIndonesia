// src/auth/model.rs

use serde::{Deserialize, Serialize};
use crate::types::UserRole; 
// This struct represents the JSON payload the user will send when logging in.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub nip: String,
    pub password: String,
}

// This struct represents the JSON payload we will send back on successful login.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// These are the "claims" we will encode into the JSON Web Token.
// It contains the user's identity and permissions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // Subject (the user's ID)
    pub role: UserRole,
    pub unit_kerja_id: Option<i32>,
    pub exp: usize, // Expiration time
}
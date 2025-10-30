use serde::{Deserialize, Serialize};
use crate::types::UserRoleEnum; 

// [FIX] Sesuaikan field agar konsisten dengan database dan frontend
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub nip_user: String, // Menggunakan nama yang sama dengan kolom DB
    pub password: String,
}

// Struct ini sudah OK
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// Struct ini sudah OK
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, 
    pub role: UserRoleEnum,
    pub bapas_id: Option<i32>,
    pub kanwil_id: Option<i32>,
    pub exp: usize, 
}

// Struct ini sudah OK
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub role: UserRoleEnum,
    pub bapas_id: Option<i32>,
    pub kanwil_id: Option<i32>,
}
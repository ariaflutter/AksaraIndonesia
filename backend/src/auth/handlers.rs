// src/auth/handlers.rs

use axum::{extract::Extension, http::StatusCode, Json};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};
// [FIX] Import AuthenticatedUser
use super::model::{AuthenticatedUser, Claims, LoginRequest, LoginResponse}; 
use crate::users::model::User;

// Secret key Anda tetap sama
const JWT_SECRET: &[u8] = b"your-super-secret-and-long-key";

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    
    // 1. Cari user berdasarkan NIP.
    // [IMPROVEMENT] Query sekarang lebih aman dan spesifik
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, nip_user, nama_user, gelar_depan_user, gelar_belakang_user,
            pangkat_golongan_user, jabatan_user, bapas_id, kanwil_id,
            status_kepegawaian_user AS "status_kepegawaian_user: _",
            email_user, nomor_telepon_user,
            status_aktif_user AS "status_aktif_user: _",
            role_user AS "role_user: _",
            password_hash, created_at, updated_at, created_by, updated_by, deleted_at
        FROM users 
        WHERE nip_user = $1 
          AND status_aktif_user = 'Aktif' 
          AND deleted_at IS NULL
        "#,
        payload.nip_user // [FIX] Gunakan nip_user dari payload
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error during login: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::UNAUTHORIZED)?; // Jika user tidak ditemukan, tidak aktif, atau sudah dihapus

    // 2. Verifikasi password (logika ini sudah benar)
    let password_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !password_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // 3. Buat JWT claims
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let claims = Claims {
        sub: user.id,
        role: user.role_user, // [FIX] Gunakan field role_user dari struct User
        bapas_id: user.bapas_id,
        kanwil_id: user.kanwil_id,
        exp: (now + 60 * 60 * 24) as usize, // Token berlaku 1 hari
    };
    
    // 4. Encode token (logika ini sudah benar)
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|e| {
            tracing::error!("Failed to create JWT: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 5. Kirim token
    Ok(Json(LoginResponse { token }))
}

pub async fn me(
    // [FIX] Middleware sekarang menyediakan AuthenticatedUser
    Extension(current_user): Extension<AuthenticatedUser>, 
    Extension(pool): Extension<PgPool>,
) -> Result<Json<User>, StatusCode> {

    // ID user ada di dalam AuthenticatedUser yang sudah divalidasi
    let user_id = current_user.id;

    // Fetch user dari database
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, nip_user, nama_user, gelar_depan_user, gelar_belakang_user,
            pangkat_golongan_user, jabatan_user, bapas_id, kanwil_id,
            status_kepegawaian_user AS "status_kepegawaian_user: _",
            email_user, nomor_telepon_user,
            status_aktif_user AS "status_aktif_user: _",
            role_user AS "role_user: _",
            password_hash, created_at, updated_at, created_by, updated_by, deleted_at
        FROM users 
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch user profile: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    // Jika user tidak ditemukan (misalnya, di-soft-delete setelah token dibuat)
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}
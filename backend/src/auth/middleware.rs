// File: src/auth/middleware.rs

use axum::{
    body::Body,
    extract::{Extension, Path}, // [FIX] Gabungkan impor dari axum::extract
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::PgPool;

use crate::auth::model::{AuthenticatedUser, Claims};
use crate::types::UserRoleEnum;

// [FIX] Impor dari modul authorization yang sekarang sudah ada
use super::authorization::{check_permission, get_klien_ownership};

const JWT_SECRET: &[u8] = b"your-super-secret-and-long-key";

// --- MIDDLEWARE UTAMA: OTENTIKASI (Authentication) ---
pub async fn auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    
    let token = req.headers()
        .get("authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "))
        .ok_or_else(|| {
            tracing::warn!("Request is missing bearer token");
            StatusCode::UNAUTHORIZED
        })?;

    let claims = decode::<Claims>(
        token,
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
        bapas_id: claims.bapas_id,
        kanwil_id: claims.kanwil_id,
    };
    req.extensions_mut().insert(authenticated_user);

    Ok(next.run(req).await)
}


// --- MIDDLEWARE OTORISASI: AKSES KLIEN (Standar) ---
pub async fn authorize_klien_access(
    Path(klien_id): Path<i32>,
    Extension(user): Extension<AuthenticatedUser>,
    Extension(pool): Extension<PgPool>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let resource_ownership = get_klien_ownership(&pool, klien_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if !check_permission(&user, &resource_ownership) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(next.run(request).await)
}


// --- MIDDLEWARE OTORISASI: AKSES LAPOR PETUGAS (Aturan Khusus) ---
pub async fn authorize_petugas_lapor_access(
    Path(klien_id): Path<i32>,
    Extension(user): Extension<AuthenticatedUser>,
    Extension(pool): Extension<PgPool>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let resource_ownership = get_klien_ownership(&pool, klien_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let has_permission = match user.role {
        UserRoleEnum::SuperAdmin | UserRoleEnum::AdminKanwil | UserRoleEnum::AdminBapas => {
            check_permission(&user, &resource_ownership)
        }
        UserRoleEnum::Pegawai => {
            // Aturan khusus: Pegawai boleh akses semua klien di Bapasnya
            user.bapas_id.is_some()
                && resource_ownership.bapas_id.is_some()
                && user.bapas_id == resource_ownership.bapas_id
        }
    };

    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(next.run(request).await)
}


// --- MIDDLEWARE OTORISASI: HAPUS WAJIB LAPOR (Aturan Khusus) ---
pub async fn authorize_wajib_lapor_delete_access(
    Path(wajib_lapor_id): Path<i64>,
    Extension(user): Extension<AuthenticatedUser>,
    Extension(pool): Extension<PgPool>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let klien_id = match sqlx::query_scalar!("SELECT klien_id FROM wajib_lapor_dewasa WHERE id = $1", wajib_lapor_id)
        .fetch_optional(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
        };
    
    let resource_ownership = get_klien_ownership(&pool, klien_id)
        .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let has_permission = match user.role {
        UserRoleEnum::SuperAdmin | UserRoleEnum::AdminKanwil | UserRoleEnum::AdminBapas => {
            check_permission(&user, &resource_ownership)
        }
        _ => false, // Pegawai tidak boleh menghapus
    };

    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(next.run(request).await)
}
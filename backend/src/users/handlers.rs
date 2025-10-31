// src/users/handlers.rs

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use rand::{distributions::Alphanumeric, Rng};
use sha256::digest;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
// [FIX] Ganti Claims dengan AuthenticatedUser
use crate::auth::model::AuthenticatedUser; 
// [FIX] Ganti UserRole dengan UserRoleEnum
use crate::types::UserRoleEnum; 
use super::model::{CreateUser, UpdateUser, User, ApiKeyStatus, NewApiKey};

// --- READ ALL ---
pub async fn get_all_users(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    // [FIX] Gunakan query_as! makro untuk keamanan tipe dan nama kolom yang benar
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, nip_user, nama_user, gelar_depan_user, gelar_belakang_user,
            pangkat_golongan_user, jabatan_user, bapas_id, kanwil_id,
            status_kepegawaian_user AS "status_kepegawaian_user: _",
            email_user, nomor_telepon_user,
            status_aktif_user AS "status_aktif_user: _",
            role_user AS "role_user: _",
            password_hash, api_key_hash,created_at, updated_at, created_by, updated_by, deleted_at
        FROM users 
        WHERE deleted_at IS NULL 
        ORDER BY nama_user
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch users: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(users))
}

// --- READ ONE ---
pub async fn get_user_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
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
            password_hash, api_key_hash,created_at, updated_at, created_by, updated_by, deleted_at
        FROM users 
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch user by id: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(user))
}

// --- CREATE ---
pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    // [FIX] Gunakan AuthenticatedUser
    Extension(current_user): Extension<AuthenticatedUser>, 
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    // Note: Logika otorisasi ini nantinya akan dipindah ke middleware.
    if current_user.role != UserRoleEnum::SuperAdmin && current_user.role != UserRoleEnum::AdminBapas {
        return Err(StatusCode::FORBIDDEN);
    }
    if current_user.role == UserRoleEnum::AdminBapas && payload.bapas_id != current_user.bapas_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let password_hash = hash(&payload.password, DEFAULT_COST).map_err(|_| {
        tracing::error!("Failed to hash password");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let api_key_hash: Option<String> = None;

let new_user = sqlx::query_as!(
    User,
    r#"
    INSERT INTO users (
        nip_user, nama_user, gelar_depan_user, gelar_belakang_user, pangkat_golongan_user,
        jabatan_user, bapas_id, kanwil_id, status_kepegawaian_user, email_user,
        nomor_telepon_user, status_aktif_user, role_user, password_hash, api_key_hash,
        created_by, updated_by
    )
    VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
        $11, $12, $13, $14, $15, $16, $17
    )
    RETURNING
        id, nip_user, nama_user, gelar_depan_user, gelar_belakang_user,
        pangkat_golongan_user, jabatan_user, bapas_id, kanwil_id,
        status_kepegawaian_user AS "status_kepegawaian_user: _",
        email_user, nomor_telepon_user,
        status_aktif_user AS "status_aktif_user: _",
        role_user AS "role_user: _",
        password_hash, api_key_hash, created_at, updated_at, created_by, updated_by, deleted_at
    "#,
    payload.nip_user,
    payload.nama_user,
    payload.gelar_depan_user,
    payload.gelar_belakang_user,
    payload.pangkat_golongan_user,
    payload.jabatan_user,
    payload.bapas_id,
    payload.kanwil_id,
    payload.status_kepegawaian_user as _,
    payload.email_user,
    payload.nomor_telepon_user,
    payload.status_aktif_user as _,
    payload.role_user as _,
    password_hash,
    api_key_hash,              // ✅ now defined
    current_user.id,           // created_by
    current_user.id            // updated_by
)
.fetch_one(&pool)
.await
    .map_err(|e| {
        tracing::error!("Failed to create user: {}", e);
        // [IMPROVEMENT] Memberi feedback jika NIP sudah ada
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return StatusCode::CONFLICT; // NIP sudah terdaftar
            }
        }
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_user))
}

// --- DELETE (SOFT DELETE) ---
pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Extension(current_user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> StatusCode {
    if current_user.role != UserRoleEnum::SuperAdmin {
        return StatusCode::FORBIDDEN;
    }
    
    // [FIX] Lakukan soft delete dengan mengisi `deleted_at` dan `updated_by`
    let result = sqlx::query!(
        "UPDATE users SET deleted_at = NOW(), updated_by = $1 WHERE id = $2",
        current_user.id,
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(query_result) => {
            if query_result.rows_affected() == 0 {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::NO_CONTENT
            }
        }
        Err(e) => {
            tracing::error!("Failed to soft delete user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}


// --- UPDATE ---
pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Extension(current_user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, StatusCode> {
    
    let user_to_update = sqlx::query_as!(User, r#"SELECT id, nip_user, nama_user, gelar_depan_user, gelar_belakang_user, pangkat_golongan_user, jabatan_user, bapas_id, kanwil_id, status_kepegawaian_user AS "status_kepegawaian_user: _", email_user, nomor_telepon_user, status_aktif_user AS "status_aktif_user: _", role_user AS "role_user: _", password_hash, api_key_hash,created_at, updated_at, created_by, updated_by, deleted_at FROM users WHERE id = $1 AND deleted_at IS NULL"#, id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Note: Logika otorisasi ini nantinya akan dipindah ke middleware.
    match current_user.role {
        UserRoleEnum::SuperAdmin => {}
        UserRoleEnum::AdminKanwil => {
            if user_to_update.kanwil_id != current_user.kanwil_id {
                return Err(StatusCode::FORBIDDEN);
            }
        },
        UserRoleEnum::AdminBapas => {
            if user_to_update.bapas_id != current_user.bapas_id {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        UserRoleEnum::Pegawai => {
            if user_to_update.id != current_user.id {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    let password_hash = match payload.password {
        Some(new_password) => hash(&new_password, DEFAULT_COST)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        None => user_to_update.password_hash,
    };

   let api_key_hash: Option<String> = None;
    
    let updated_user = sqlx::query_as!(
        User,
        r#"
        UPDATE users SET
            nip_user = COALESCE($1, nip_user),
            nama_user = COALESCE($2, nama_user),
            gelar_depan_user = COALESCE($3, gelar_depan_user),
            gelar_belakang_user = COALESCE($4, gelar_belakang_user),
            pangkat_golongan_user = COALESCE($5, pangkat_golongan_user),
            jabatan_user = COALESCE($6, jabatan_user),
            bapas_id = COALESCE($7, bapas_id),
            kanwil_id = COALESCE($8, kanwil_id),
            status_kepegawaian_user = COALESCE($9, status_kepegawaian_user),
            email_user = COALESCE($10, email_user),
            nomor_telepon_user = COALESCE($11, nomor_telepon_user),
            status_aktif_user = COALESCE($12, status_aktif_user),
            role_user = COALESCE($13, role_user),
            password_hash = $14,
            api_key_hash = $15,
            updated_by = $16
        WHERE id = $17
        RETURNING
            id, nip_user, nama_user, gelar_depan_user, gelar_belakang_user,
            pangkat_golongan_user, jabatan_user, bapas_id, kanwil_id,
            status_kepegawaian_user AS "status_kepegawaian_user: _",
            email_user, nomor_telepon_user,
            status_aktif_user AS "status_aktif_user: _",
            role_user AS "role_user: _",
            password_hash, api_key_hash,created_at, updated_at, created_by, updated_by, deleted_at
        "#,
        payload.nip_user,
        payload.nama_user,
        payload.gelar_depan_user,
        payload.gelar_belakang_user,
        payload.pangkat_golongan_user,
        payload.jabatan_user,
        payload.bapas_id,
        payload.kanwil_id,
        payload.status_kepegawaian_user as _,
        payload.email_user,
        payload.nomor_telepon_user,
        payload.status_aktif_user as _,
        payload.role_user as _,
        password_hash,
        api_key_hash,
        current_user.id, // [FIX] Mengisi updated_by
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(updated_user))
}

// --- GET API KEY STATUS ---
// URL: GET /api/me/api-key
pub async fn get_my_api_key_status(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<ApiKeyStatus>, StatusCode> {
    let has_key = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1 AND api_key_hash IS NOT NULL)",
        user.id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(false);

    Ok(Json(ApiKeyStatus { has_key }))
}


// --- GENERATE/REGENERATE API KEY ---
// URL: POST /api/me/api-key
pub async fn generate_my_api_key(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<NewApiKey>, StatusCode> {
    // 1. Generate token acak yang aman
    let new_key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32) // Panjang token
        .map(char::from)
        .collect();
    let api_key = format!("ak_{}", new_key); // Tambahkan prefix

    // 2. Hash token tersebut
    let key_hash = digest(api_key.clone());

    // 3. Update database
    sqlx::query!(
        "UPDATE users SET api_key_hash = $1 WHERE id = $2",
        key_hash,
        user.id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 4. Kembalikan key asli ke pengguna
    Ok(Json(NewApiKey { api_key }))
}

// --- DELETE API KEY ---
// URL: DELETE /api/me/api-key
pub async fn delete_my_api_key(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> StatusCode {
    sqlx::query!(
        "UPDATE users SET api_key_hash = NULL WHERE id = $1",
        user.id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    .map(|_| StatusCode::NO_CONTENT)
    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
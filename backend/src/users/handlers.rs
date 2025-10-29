// src/users/handlers.rs

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use crate::auth::model::Claims;
use crate::types::UserRole;
use super::model::{CreateUser, User, UpdateUser};
// <-- Add this to your imports at the top



// --- READ ALL ---
pub async fn get_all_users(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY nama")
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
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
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
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    // Authorization: Only SuperAdmin or AdminBapas can create users.
    if claims.role != UserRole::SuperAdmin && claims.role != UserRole::AdminBapas {
        return Err(StatusCode::FORBIDDEN);
    }
    // More specific rule: An AdminBapas can only create users for their own Unit Kerja.
    if claims.role == UserRole::AdminBapas && payload.unit_kerja_id != claims.unit_kerja_id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Hash the password before storing.
    let password_hash = hash(&payload.password, DEFAULT_COST).map_err(|_| {
        tracing::error!("Failed to hash password");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let new_user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (
            nip, nama, gelar_depan, gelar_belakang, pangkat_golongan, jabatan,
            unit_kerja_id, status_kepegawaian, email, nomor_telepon,
            status_aktif, role, password_hash
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING
            id, nip, nama, gelar_depan, gelar_belakang, pangkat_golongan, jabatan,
            unit_kerja_id, kanwil_id,status_kepegawaian AS "status_kepegawaian: _", email, nomor_telepon,
            status_aktif AS "status_aktif: _", role AS "role: _", password_hash
        "#,
        payload.nip,
        payload.nama,
        payload.gelar_depan,
        payload.gelar_belakang,
        payload.pangkat_golongan,
        payload.jabatan,
        payload.unit_kerja_id,
        payload.status_kepegawaian as _,
        payload.email,
        payload.nomor_telepon,
        payload.status_aktif as _,
        payload.role as _,
        password_hash
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_user))
}

// --- DELETE ---
pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> StatusCode {
    // Authorization: Let's say only SuperAdmin can delete.
    if claims.role != UserRole::SuperAdmin {
        return StatusCode::FORBIDDEN;
    }

    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
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
            tracing::error!("Failed to delete user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

// Note: The UPDATE handler is more complex due to optional fields
// and conditional password hashing. We will add it after these are working.

// in src/users/handlers.rs
// ... (at the bottom of the file)


// --- UPDATE ---
pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, StatusCode> {
    
    // Authorization: A SuperAdmin can update anyone. An AdminBapas can only update users
    // in their own Unit Kerja. A Pegawai can only update their own profile.
    
    // 1. Fetch the user being updated to check their current unit_kerja_id
    let user_to_update = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 2. Perform Authorization Checks
    match claims.role {
        UserRole::SuperAdmin => { /* SuperAdmin can do anything, proceed */ }
        UserRole::AdminKanwil => {
            if user_to_update.kanwil_id != claims.kanwil_id {
                return Err(StatusCode::FORBIDDEN);
            }
        },

        UserRole::AdminBapas => {
            // AdminBapas can only update users in their own unit.
            if user_to_update.unit_kerja_id != claims.unit_kerja_id {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        UserRole::Pegawai => {
            // A Pegawai can only update their own record (id must match).
            if user_to_update.id != claims.sub {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    // 3. Handle password update
    // If a new password is provided in the payload, hash it.
    // Otherwise, keep the existing password_hash from the database.
    let password_hash = match payload.password {
        Some(new_password) => hash(&new_password, DEFAULT_COST).map_err(|_| {
            tracing::error!("Failed to hash new password for user {}", id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?,
        None => user_to_update.password_hash,
    };
    
    // 4. Perform the database update
    // Use COALESCE($n, existing_column) to only update fields that are provided
    // in the payload. If a field in the payload is None, the existing value is kept.
    let updated_user = sqlx::query_as!(
        User,
        r#"
        UPDATE users SET
            nip = COALESCE($1, nip),
            nama = COALESCE($2, nama),
            gelar_depan = COALESCE($3, gelar_depan),
            gelar_belakang = COALESCE($4, gelar_belakang),
            pangkat_golongan = COALESCE($5, pangkat_golongan),
            jabatan = COALESCE($6, jabatan),
            unit_kerja_id = COALESCE($7, unit_kerja_id),
            kanwil_id = COALESCE($8, kanwil_id),
            status_kepegawaian = COALESCE($9, status_kepegawaian),
            email = COALESCE($10, email),
            nomor_telepon = COALESCE($11, nomor_telepon),
            status_aktif = COALESCE($12, status_aktif),
            role = COALESCE($13, role),
            password_hash = $14
        WHERE id = $15
        RETURNING
            id, nip, nama, gelar_depan, gelar_belakang, pangkat_golongan, jabatan,
            unit_kerja_id, kanwil_id,status_kepegawaian AS "status_kepegawaian: _", email, nomor_telepon,
            status_aktif AS "status_aktif: _", role AS "role: _", password_hash
        "#,
        payload.nip,
        payload.nama,
        payload.gelar_depan,
        payload.gelar_belakang,
        payload.pangkat_golongan,
        payload.jabatan,
        payload.unit_kerja_id,
        payload.kanwil_id,
        payload.status_kepegawaian as _,
        payload.email,
        payload.nomor_telepon,
        payload.status_aktif as _,
        payload.role as _,
        password_hash,
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
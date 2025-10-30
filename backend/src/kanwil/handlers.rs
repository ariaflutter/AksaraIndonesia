// File baru: src/kanwil/handlers.rs

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::auth::model::AuthenticatedUser;
use crate::types::UserRoleEnum;
use super::model::{CreateKanwil, Kanwil, UpdateKanwil};

// --- CREATE ---
pub async fn create_kanwil(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(payload): Json<CreateKanwil>,
) -> Result<Json<Kanwil>, StatusCode> {
    if user.role != UserRoleEnum::SuperAdmin {
        return Err(StatusCode::FORBIDDEN);
    }

    let new_kanwil = sqlx::query_as!(
        Kanwil,
        r#"
        INSERT INTO kanwil (nama_kanwil, alamat_kanwil, nomor_telepon_kanwil, email_kanwil)
        VALUES ($1, $2, $3, $4)
        RETURNING id, nama_kanwil, alamat_kanwil, nomor_telepon_kanwil, email_kanwil, created_at, updated_at, deleted_at
        "#,
        payload.nama_kanwil,
        payload.alamat_kanwil,
        payload.nomor_telepon_kanwil,
        payload.email_kanwil
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create kanwil: {}", e);
        if e.as_database_error().map_or(false, |db_err| db_err.is_unique_violation()) {
            return StatusCode::CONFLICT;
        }
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_kanwil))
}

// --- READ ALL ---
pub async fn get_all_kanwil(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<Vec<Kanwil>>, StatusCode> {
    if user.role != UserRoleEnum::SuperAdmin {
        return Err(StatusCode::FORBIDDEN);
    }

    let kanwils = sqlx::query_as!(
        Kanwil,
        r#"
        SELECT id, nama_kanwil, alamat_kanwil, nomor_telepon_kanwil, email_kanwil, created_at, updated_at, deleted_at
        FROM kanwil WHERE deleted_at IS NULL ORDER BY nama_kanwil
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(kanwils))
}

// --- READ ONE ---
pub async fn get_kanwil_by_id(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> Result<Json<Kanwil>, StatusCode> {
    if user.role != UserRoleEnum::SuperAdmin {
        return Err(StatusCode::FORBIDDEN);
    }

    let kanwil = sqlx::query_as!(
        Kanwil,
        r#"
        SELECT id, nama_kanwil, alamat_kanwil, nomor_telepon_kanwil, email_kanwil, created_at, updated_at, deleted_at
        FROM kanwil WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(kanwil))
}

// --- UPDATE ---
pub async fn update_kanwil(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateKanwil>,
) -> Result<Json<Kanwil>, StatusCode> {
    if user.role != UserRoleEnum::SuperAdmin {
        return Err(StatusCode::FORBIDDEN);
    }

    let updated_kanwil = sqlx::query_as!(
        Kanwil,
        r#"
        UPDATE kanwil
        SET 
            nama_kanwil = COALESCE($1, nama_kanwil),
            alamat_kanwil = COALESCE($2, alamat_kanwil),
            nomor_telepon_kanwil = COALESCE($3, nomor_telepon_kanwil),
            email_kanwil = COALESCE($4, email_kanwil)
        WHERE id = $5 AND deleted_at IS NULL
        RETURNING id, nama_kanwil, alamat_kanwil, nomor_telepon_kanwil, email_kanwil, created_at, updated_at, deleted_at
        "#,
        payload.nama_kanwil,
        payload.alamat_kanwil,
        payload.nomor_telepon_kanwil,
        payload.email_kanwil,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update kanwil: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(updated_kanwil))
}

// --- DELETE (SOFT) ---
pub async fn delete_kanwil(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> StatusCode {
    if user.role != UserRoleEnum::SuperAdmin {
        return StatusCode::FORBIDDEN;
    }

    let result = sqlx::query!("UPDATE kanwil SET deleted_at = NOW() WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
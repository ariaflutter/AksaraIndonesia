// File baru: src/bapas/handlers.rs

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::auth::model::AuthenticatedUser;
use crate::types::UserRoleEnum;
use super::model::{Bapas, CreateBapas, UpdateBapas};

// --- CREATE ---
pub async fn create_bapas(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(payload): Json<CreateBapas>,
) -> Result<Json<Bapas>, StatusCode> {
    // Otorisasi: SuperAdmin atau AdminKanwil yang sesuai
    match user.role {
        UserRoleEnum::SuperAdmin => {} // Boleh
        UserRoleEnum::AdminKanwil => {
            // AdminKanwil hanya boleh membuat Bapas di dalam Kanwilnya.
            if user.kanwil_id != Some(payload.kanwil_id) {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        _ => return Err(StatusCode::FORBIDDEN),
    }

    let new_bapas = sqlx::query_as!(
        Bapas,
        r#"
        INSERT INTO bapas (kanwil_id, nama_bapas, kota_bapas, alamat_bapas, nomor_telepon_bapas, email_bapas)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, kanwil_id, nama_bapas, kota_bapas, alamat_bapas, nomor_telepon_bapas, email_bapas, created_at, updated_at, deleted_at
        "#,
        payload.kanwil_id,
        payload.nama_bapas,
        payload.kota_bapas,
        payload.alamat_bapas,
        payload.nomor_telepon_bapas,
        payload.email_bapas
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if e.as_database_error().map_or(false, |db_err| db_err.is_unique_violation()) {
            return StatusCode::CONFLICT;
        }
        tracing::error!("Failed to create bapas: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_bapas))
}

// --- READ ALL ---
// Handler ini akan menampilkan semua Bapas ke SuperAdmin,
// tapi hanya Bapas yang relevan untuk AdminKanwil.
pub async fn get_all_bapas(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<Vec<Bapas>>, StatusCode> {
    let bapas_list = match user.role {
        UserRoleEnum::SuperAdmin => {
            sqlx::query_as!(Bapas, "SELECT * FROM bapas WHERE deleted_at IS NULL ORDER BY nama_bapas")
                .fetch_all(&pool)
                .await
        }
        UserRoleEnum::AdminKanwil => {
            sqlx::query_as!(
                Bapas, 
                "SELECT * FROM bapas WHERE kanwil_id = $1 AND deleted_at IS NULL ORDER BY nama_bapas",
                user.kanwil_id
            )
            .fetch_all(&pool)
            .await
        }
        // AdminBapas dan Pegawai mungkin perlu melihat daftar Bapas, kita izinkan (read-only)
        UserRoleEnum::AdminBapas | UserRoleEnum::Pegawai => {
             sqlx::query_as!(Bapas, "SELECT * FROM bapas WHERE deleted_at IS NULL ORDER BY nama_bapas")
                .fetch_all(&pool)
                .await
        }
    };
    
    match bapas_list {
        Ok(list) => Ok(Json(list)),
        Err(e) => {
            tracing::error!("Failed to fetch bapas list: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


// --- READ ONE ---
pub async fn get_bapas_by_id(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> Result<Json<Bapas>, StatusCode> {
    let bapas = sqlx::query_as!(
        Bapas,
        "SELECT * FROM bapas WHERE id = $1 AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Otorisasi: Pastikan AdminKanwil hanya mengakses Bapas di wilayahnya.
    match user.role {
        UserRoleEnum::SuperAdmin => Ok(Json(bapas)),
        UserRoleEnum::AdminKanwil => {
            if user.kanwil_id == Some(bapas.kanwil_id) {
                Ok(Json(bapas))
            } else {
                Err(StatusCode::FORBIDDEN)
            }
        }
        _ => Ok(Json(bapas)), // Izinkan read untuk role lain
    }
}

// --- UPDATE ---
pub async fn update_bapas(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBapas>,
) -> Result<Json<Bapas>, StatusCode> {
    // Pertama, fetch Bapas yang akan diupdate untuk memeriksa kepemilikan
    let bapas_to_update = sqlx::query_as!(Bapas, "SELECT * FROM bapas WHERE id = $1", id)
        .fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    match user.role {
        UserRoleEnum::SuperAdmin => {},
        UserRoleEnum::AdminKanwil => {
            if user.kanwil_id != Some(bapas_to_update.kanwil_id) {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        _ => return Err(StatusCode::FORBIDDEN),
    }

    let updated_bapas = sqlx::query_as!(
        Bapas,
        r#"
        UPDATE bapas
        SET 
            kanwil_id = COALESCE($1, kanwil_id),
            nama_bapas = COALESCE($2, nama_bapas),
            kota_bapas = COALESCE($3, kota_bapas),
            alamat_bapas = COALESCE($4, alamat_bapas),
            nomor_telepon_bapas = COALESCE($5, nomor_telepon_bapas),
            email_bapas = COALESCE($6, email_bapas)
        WHERE id = $7 AND deleted_at IS NULL
        RETURNING id, kanwil_id, nama_bapas, kota_bapas, alamat_bapas, nomor_telepon_bapas, email_bapas, created_at, updated_at, deleted_at
        "#,
        payload.kanwil_id,
        payload.nama_bapas,
        payload.kota_bapas,
        payload.alamat_bapas,
        payload.nomor_telepon_bapas,
        payload.email_bapas,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(updated_bapas))
}

// --- DELETE (SOFT) ---
pub async fn delete_bapas(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> StatusCode {
     // Fetch dulu untuk otorisasi
    let bapas_to_delete = match sqlx::query_as!(Bapas, "SELECT * FROM bapas WHERE id = $1", id)
        .fetch_optional(&pool).await {
            Ok(Some(bapas)) => bapas,
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR
        };

    match user.role {
        UserRoleEnum::SuperAdmin => {},
        UserRoleEnum::AdminKanwil => {
            if user.kanwil_id != Some(bapas_to_delete.kanwil_id) {
                return StatusCode::FORBIDDEN;
            }
        }
        _ => return StatusCode::FORBIDDEN,
    }

    let result = sqlx::query!("UPDATE bapas SET deleted_at = NOW() WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
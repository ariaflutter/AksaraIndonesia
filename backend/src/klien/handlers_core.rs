// src/klien/handlers_core.rs

use crate::auth::model::Claims;
use crate::klien::model_core::{CreateKlien, Klien, UpdateKlien};
use axum::{extract::Extension, http::StatusCode, Json};
use sqlx::PgPool;
use axum::extract::{Path, Query};
use crate::utils::Pagination; // Make sure you created this file in the last step

/// API handler to create a new core Klien identity.
pub async fn create_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateKlien>,
) -> Result<Json<Klien>, StatusCode> {
    
    let created_by_id = claims.sub;

    // --- REVISED: AUTHORIZATION AND DATA SCOPING LOGIC ---
    let target_bapas_id: i32;
    let target_pk_id: i32;

    match claims.role {
        crate::types::UserRole::SuperAdmin => {
            // A SuperAdmin MUST provide both a bapas_id and a pk_id in the payload.
            target_bapas_id = payload.bapas_id.ok_or(StatusCode::BAD_REQUEST)?;
            target_pk_id = payload.pk_id.ok_or(StatusCode::BAD_REQUEST)?;
        }
        crate::types::UserRole::AdminBapas => {
            // An AdminBapas can assign to any PK within their own Bapas.
            // They MUST provide a pk_id.
            // The bapas_id is taken from their token for security.
            target_bapas_id = claims.unit_kerja_id.ok_or(StatusCode::FORBIDDEN)?;
            target_pk_id = payload.pk_id.ok_or(StatusCode::BAD_REQUEST)?;
            // (In a real app, you would add a check here to ensure the target_pk_id
            //  actually belongs to the target_bapas_id).
        }
        crate::types::UserRole::Pegawai => {
            // A Pegawai has no choice.
            // The client is assigned to THEIR Bapas and to THEM.
            // We completely ignore the payload's bapas_id and pk_id.
            target_bapas_id = claims.unit_kerja_id.ok_or(StatusCode::FORBIDDEN)?;
            target_pk_id = claims.sub; // The client is assigned to the user creating it.
        }
    }
    // ---------------------------------------------------

    // Now, we proceed with the insertion, but we use our verified `target_bapas_id`.
    let result = sqlx::query_as(
        r#"
        INSERT INTO klien (
            tipe, nama, alamat, tempat_lahir, tanggal_lahir, jenis_kelamin, agama,
            pekerjaan, pendidikan_terakhir, bapas_id, pk_id, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
    )
    .bind(&payload.tipe)
    .bind(&payload.nama)
    .bind(&payload.alamat)
    .bind(&payload.tempat_lahir)
    .bind(payload.tanggal_lahir)
    .bind(&payload.jenis_kelamin)
    .bind(&payload.agama)
    .bind(&payload.pekerjaan)
    .bind(payload.pendidikan_terakhir)
    .bind(target_bapas_id) // <-- USE THE VERIFIED ID, NOT THE PAYLOAD ID
    .bind(target_pk_id)
    .bind(created_by_id)
    .bind(created_by_id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(new_klien) => Ok(Json(new_klien)),
        Err(e) => {
            tracing::error!("Failed to create klien: {}", e);
            // Check for specific foreign key violations, e.g., if pk_id or bapas_id doesn't exist.
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_foreign_key_violation() {
                    return Err(StatusCode::BAD_REQUEST); // 400 Bad Request
                }
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


// --- READ ALL (with Pagination) ---
pub async fn get_all_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>, // <-- We now need the user's claims
    pagination: Query<Pagination>,
) -> Result<Json<Vec<Klien>>, StatusCode> {
    
    let offset = (pagination.page - 1) * pagination.limit;

    // --- NEW: DYNAMIC QUERY BUILDING BASED ON ROLE ---

    // Start with the base query.
      let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT
            id,
            tipe,
            nama,
            alamat,
            tempat_lahir,
            tanggal_lahir,
            jenis_kelamin,
            agama,
            pekerjaan,
            pendidikan_terakhir,
            bapas_id,
            pk_id,
            created_at,
            updated_at,
            created_by,
            updated_by
        FROM klien
        "#
    );

    // Dynamically add a WHERE clause based on the user's role.
    match claims.role {
        crate::types::UserRole::SuperAdmin => {
            // SuperAdmin sees everyone. No WHERE clause is added.
        }
        crate::types::UserRole::AdminBapas => {
            // AdminBapas sees all clients within their unit_kerja_id.
            if let Some(unit_kerja_id) = claims.unit_kerja_id {
                query_builder.push(" WHERE bapas_id = ");
                query_builder.push_bind(unit_kerja_id);
            } else {
                // This AdminBapas is not associated with any Bapas, which is an error state.
                // Return an empty list for security.
                return Ok(Json(Vec::new()));
            }
        }
        crate::types::UserRole::Pegawai => {
            // A Pegawai only sees clients directly assigned to them.
            query_builder.push(" WHERE pk_id = ");
            query_builder.push_bind(claims.sub); // claims.sub is the user's own ID
        }
    }

    // Add the final ordering and pagination to the query.
    query_builder.push(" ORDER BY nama LIMIT ");
    query_builder.push_bind(pagination.limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    // ---------------------------------------------------

    // Build the final query
    let query = query_builder.build_query_as::<Klien>();

    let klien_list = query
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch all klien: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(klien_list))
}

// --- READ ONE ---
pub async fn get_klien_by_id(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>, // <-- We need the user's claims
    Path(id): Path<i32>,
) -> Result<Json<Klien>, StatusCode> {
    
    // First, fetch the client from the database
    let klien = sqlx::query_as::<_, Klien>("SELECT * FROM klien WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch klien by id: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // --- NEW: AUTHORIZATION CHECK ---
    let authorized = match claims.role {
        crate::types::UserRole::SuperAdmin => true, // SuperAdmin is always authorized
        crate::types::UserRole::AdminBapas => {
            // AdminBapas is authorized if the client's bapas_id matches their own
            klien.bapas_id == claims.unit_kerja_id.unwrap_or(-1) // Use a non-existent ID if admin has no bapas
        }
        crate::types::UserRole::Pegawai => {
            // Pegawai is authorized if the client's pk_id matches their own user id
            klien.pk_id == claims.sub
        }
    };

    if !authorized {
        // If not authorized, return a 403 Forbidden error.
        // This is different from a 404; it means "the resource exists, but you can't see it."
        return Err(StatusCode::FORBIDDEN);
    }
    // ---------------------------------

    Ok(Json(klien))
}

// --- DELETE ---
pub async fn delete_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> StatusCode {
    
    // 1. Fetch the client to check ownership before deleting.
    let klien_to_delete = match sqlx::query!("SELECT bapas_id, pk_id FROM klien WHERE id = $1", id)
        .fetch_optional(&pool)
        .await 
    {
        Ok(Some(record)) => record,
        Ok(None) => return StatusCode::NOT_FOUND, // If it doesn't exist, return 404
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // 2. Perform authorization check.
    let authorized = match claims.role {
        crate::types::UserRole::SuperAdmin => true,
        crate::types::UserRole::AdminBapas => klien_to_delete.bapas_id == claims.unit_kerja_id.unwrap_or(-1),
        crate::types::UserRole::Pegawai => klien_to_delete.pk_id == claims.sub,
    };

    if !authorized {
        return StatusCode::FORBIDDEN;
    }

    // 3. If authorized, proceed with the deletion.
    let result = sqlx::query!("DELETE FROM klien WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Failed to delete klien: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn update_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateKlien>,
) -> Result<Json<Klien>, StatusCode> {
    
    // 1. Fetch the existing client to perform authorization checks on it.
    let existing_klien = sqlx::query_as::<_, Klien>("SELECT * FROM klien WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 2. Perform the same authorization check as the GET by ID handler.
    let authorized = match claims.role {
        crate::types::UserRole::SuperAdmin => true,
        crate::types::UserRole::AdminBapas => existing_klien.bapas_id == claims.unit_kerja_id.unwrap_or(-1),
        crate::types::UserRole::Pegawai => existing_klien.pk_id == claims.sub,
    };

    if !authorized {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let updated_by_id = claims.sub;

    let result = sqlx::query_as(
        r#"
        UPDATE klien SET
            tipe = COALESCE($1, tipe),
            nama = COALESCE($2, nama),
            alamat = COALESCE($3, alamat),
            tempat_lahir = COALESCE($4, tempat_lahir),
            tanggal_lahir = COALESCE($5, tanggal_lahir),
            jenis_kelamin = COALESCE($6, jenis_kelamin),
            agama = COALESCE($7, agama),
            pekerjaan = COALESCE($8, pekerjaan),
            pendidikan_terakhir = COALESCE($9, pendidikan_terakhir),
            bapas_id = COALESCE($10, bapas_id),
            pk_id = COALESCE($11, pk_id),
             online_akses = COALESCE($12, online_akses), -- <-- ADD THIS LINE
            updated_by = $13
        WHERE id = $14
        RETURNING *
        "#,
    )
    .bind(payload.tipe)
    .bind(payload.nama)
    .bind(payload.alamat)
    .bind(payload.tempat_lahir)
    .bind(payload.tanggal_lahir)
    .bind(payload.jenis_kelamin)
    .bind(payload.agama)
    .bind(payload.pekerjaan)
    .bind(payload.pendidikan_terakhir)
    .bind(payload.bapas_id)
    .bind(payload.pk_id)
    .bind(payload.online_akses)
    .bind(updated_by_id)
    .bind(id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(updated_klien) => Ok(Json(updated_klien)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update klien: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
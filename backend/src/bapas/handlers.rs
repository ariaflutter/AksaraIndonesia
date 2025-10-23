// src/bapas/handlers.rs

use axum::{extract::{Extension,Path}, http::StatusCode, Json};
use sqlx::PgPool;
use super::model::{Bapas,CreateBapas}; // Import the Bapas model from our sibling module.
use crate::types::UserRole; // <-- Import UserRole for authorization
use crate::auth::model::Claims; // <-- Import Claims to get the logged-in user

/// API handler to fetch a list of all Bapas offices.
pub async fn get_all_bapas(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Bapas>>, StatusCode> {
    
    let query = "SELECT id, nama_bapas, kota, alamat, nomor_telepon_bapas, email, kanwil FROM bapas ORDER BY nama_bapas";

    let bapas_list = sqlx::query_as::<_, Bapas>(query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch bapas data: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(bapas_list))
}

pub async fn create_bapas(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>, // Get the claims of the logged-in user
    Json(payload): Json<CreateBapas>,
) -> Result<Json<Bapas>, StatusCode> {

    // --- 1. Authorization ---
    // Check if the user's role is authorized to perform this action.
    if claims.role != UserRole::SuperAdmin && claims.role != UserRole::AdminBapas {
        tracing::warn!("Unauthorized attempt to create Bapas by user {}", claims.sub);
        return Err(StatusCode::FORBIDDEN); // 403 Forbidden
    }

    // --- 2. Validation (Simple Example) ---
    // In a real app, you'd add more validation (e.g., is the email valid?).
    if payload.nama_bapas.is_empty() {
        return Err(StatusCode::BAD_REQUEST); // 400 Bad Request
    }

    // --- 3. Database Insertion ---
    // The `RETURNING *` part is a PostgreSQL feature that returns the entire
    // newly created row, which we can then send back to the user.
    let new_bapas = sqlx::query_as::<_, Bapas>(
        "INSERT INTO bapas (nama_bapas, kota, alamat, nomor_telepon_bapas, email) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&payload.nama_bapas)
    .bind(&payload.kota)
    .bind(&payload.alamat)
    .bind(&payload.nomor_telepon_bapas)
    .bind(&payload.email)
    .bind(&payload.kanwil)
    .fetch_one(&pool) // Use `fetch_one` because RETURNING * will always return exactly one row.
    .await
    .map_err(|e| {
        tracing::error!("Failed to create bapas: {}", e);
        // You could check for specific DB errors here, like a UNIQUE constraint violation.
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // --- 4. Return the new record ---
    Ok(Json(new_bapas))
}

pub async fn get_bapas_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>, // Axum extracts the ID from the URL path into this variable.
) -> Result<Json<Bapas>, StatusCode> {
    
    let query = "SELECT * FROM bapas WHERE id = $1";

    // Use `fetch_optional` which returns an Option<Bapas>.
    // This correctly handles the case where no Bapas with the given ID is found.
    let bapas = sqlx::query_as::<_, Bapas>(query)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch bapas by id: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?; // If the Option is None, map it to a 404 Not Found.

    Ok(Json(bapas))
}

pub async fn update_bapas(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateBapas>, // We can reuse the CreateBapas struct for updates
) -> Result<Json<Bapas>, StatusCode> {
    
    // Authorization: Only SuperAdmin and AdminBapas can update.
    if claims.role != UserRole::SuperAdmin && claims.role != UserRole::AdminBapas {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // In a real app, you might also add logic here for an AdminBapas:
    // "An AdminBapas can only update the Bapas that they belong to."
    // if claims.role == UserRole::AdminBapas && claims.unit_kerja_id != Some(id) {
    //     return Err(StatusCode::FORBIDDEN);
    // }

    let query = "UPDATE bapas SET nama_bapas = $1, kota = $2, alamat = $3, nomor_telepon_bapas = $4, email = $5, kanwil = $6 WHERE id = $7 RETURNING *";

    let updated_bapas = sqlx::query_as::<_, Bapas>(query)
        .bind(&payload.nama_bapas)
        .bind(&payload.kota)
        .bind(&payload.alamat)
        .bind(&payload.nomor_telepon_bapas)
        .bind(&payload.email)
        .bind(&payload.kanwil) // You have this as an Option, so bind it.
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update bapas: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(updated_bapas))
}

pub async fn delete_bapas(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> StatusCode {

    // Authorization: For deletion, let's say ONLY a SuperAdmin can do it.
    if claims.role != UserRole::SuperAdmin {
        return StatusCode::FORBIDDEN;
    }

    let query = "DELETE FROM bapas WHERE id = $1";

    // Use `execute` as we don't need to return the deleted row.
    // It returns a `Result<PgQueryResult, _>`
    let result = sqlx::query(query)
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(query_result) => {
            // If `rows_affected` is 0, it means no Bapas with that ID was found.
            if query_result.rows_affected() == 0 {
                StatusCode::NOT_FOUND
            } else {
                // Return 204 No Content on successful deletion.
                StatusCode::NO_CONTENT
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete bapas: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
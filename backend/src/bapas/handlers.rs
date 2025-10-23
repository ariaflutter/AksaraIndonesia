// src/bapas/handlers.rs

use axum::{extract::Extension, http::StatusCode, Json};
use sqlx::PgPool;
use super::model::Bapas; // Import the Bapas model from our sibling module.

/// API handler to fetch a list of all Bapas offices.
pub async fn get_all_bapas(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Bapas>>, StatusCode> {
    
    let query = "SELECT id, nama_bapas, kota, alamat, nomor_telepon_bapas, email FROM bapas ORDER BY nama_bapas";

    let bapas_list = sqlx::query_as::<_, Bapas>(query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch bapas data: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(bapas_list))
}
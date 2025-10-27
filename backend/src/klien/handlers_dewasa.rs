// src/klien/handlers_dewasa.rs

use crate::auth::model::Claims;
use crate::klien::model_dewasa::{
    CreatePenerimaanDewasa, PenerimaanDewasa, UpdatePenerimaanDewasa,
};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

/// Checks if a logged-in user is authorized to access a specific klien.
async fn authorize_user_for_klien(
    pool: &PgPool,
    klien_id: i32,
    claims: &Claims,
) -> Result<(), StatusCode> {
    if claims.role == crate::types::UserRole::SuperAdmin {
        return Ok(());
    }
    let ownership = match sqlx::query!(
        "SELECT bapas_id, pk_id FROM klien WHERE id = $1",
        klien_id
    )
    .fetch_optional(pool)
    .await
    {
        Ok(Some(record)) => record,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let authorized = match claims.role {
        crate::types::UserRole::SuperAdmin => true,
        crate::types::UserRole::AdminBapas => ownership.bapas_id == claims.unit_kerja_id.unwrap_or(-1),
        crate::types::UserRole::Pegawai => ownership.pk_id == claims.sub,
    };
    if authorized { Ok(()) } else { Err(StatusCode::FORBIDDEN) }
}

// --- CREATE ---
pub async fn create_penerimaan_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreatePenerimaanDewasa>,
) -> Result<Json<PenerimaanDewasa>, StatusCode> {
    authorize_user_for_klien(&pool, klien_id, &claims).await?;
    
    let created_by = claims.sub;

    let result = sqlx::query_as(
        r#"
        INSERT INTO penerimaan_dewasa (
            klien_id, tanggal_permintaan_lapas, tanggal_surat_tugas, perihal,
            no_register_litmas, nomor_surat_permintaan_lapas, jenis_permintaan_litmas_lapas,
            nama_instansi, kelas_instansi, daerah_instansi, nama_penjamin, alamat_penjamin,
            created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING *
        "#,
    )
    .bind(klien_id)
    .bind(payload.tanggal_permintaan_lapas)
    .bind(payload.tanggal_surat_tugas)
    .bind(payload.perihal)
    .bind(payload.no_register_litmas)
    .bind(payload.nomor_surat_permintaan_lapas)
    .bind(payload.jenis_permintaan_litmas_lapas)
    .bind(payload.nama_instansi)
    .bind(payload.kelas_instansi)
    .bind(payload.daerah_instansi)
    .bind(payload.nama_penjamin)
    .bind(payload.alamat_penjamin)
    .bind(created_by)
    .bind(created_by)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => Ok(Json(record)),
        Err(e) => {
            tracing::error!("Failed to create penerimaan_dewasa: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// --- READ ALL ---
pub async fn get_all_penerimaan_for_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(klien_id): Path<i32>,
) -> Result<Json<Vec<PenerimaanDewasa>>, StatusCode> {
    authorize_user_for_klien(&pool, klien_id, &claims).await?;
    let records = sqlx::query_as("SELECT * FROM penerimaan_dewasa WHERE klien_id = $1")
        .bind(klien_id).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(records))
}

// --- READ ONE ---
pub async fn get_penerimaan_by_id(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<PenerimaanDewasa>, StatusCode> {
    let record: PenerimaanDewasa = sqlx::query_as("SELECT * FROM penerimaan_dewasa WHERE id = $1")
        .bind(id).fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    authorize_user_for_klien(&pool, record.klien_id, &claims).await?;
    Ok(Json(record))
}

// --- UPDATE ---
pub async fn update_penerimaan_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>, // This is the ID of the penerimaan_dewasa record
    Json(payload): Json<UpdatePenerimaanDewasa>,
) -> Result<Json<PenerimaanDewasa>, StatusCode> {
    
    // First, fetch the record to find its parent klien_id for authorization
    let klien_id = match sqlx::query_scalar!("SELECT klien_id FROM penerimaan_dewasa WHERE id = $1", id)
        .fetch_optional(&pool).await {
        Ok(Some(id)) => id,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Authorize the user against the parent client
    authorize_user_for_klien(&pool, klien_id, &claims).await?;
    
    let updated_by = claims.sub;

    let result = sqlx::query_as(
        r#"
        UPDATE penerimaan_dewasa
        SET
            tanggal_permintaan_lapas = COALESCE($1, tanggal_permintaan_lapas),
            tanggal_surat_tugas = COALESCE($2, tanggal_surat_tugas),
            perihal = COALESCE($3, perihal),
            no_register_litmas = COALESCE($4, no_register_litmas),
            nomor_surat_permintaan_lapas = COALESCE($5, nomor_surat_permintaan_lapas),
            jenis_permintaan_litmas_lapas = COALESCE($6, jenis_permintaan_litmas_lapas),
            nama_instansi = COALESCE($7, nama_instansi),
            kelas_instansi = COALESCE($8, kelas_instansi),
            daerah_instansi = COALESCE($9, daerah_instansi),
            nama_penjamin = COALESCE($10, nama_penjamin),
            alamat_penjamin = COALESCE($11, alamat_penjamin),
            updated_by = $12
        WHERE id = $13
        RETURNING *
        "#,
    )
    .bind(payload.tanggal_permintaan_lapas)
    .bind(payload.tanggal_surat_tugas)
    .bind(payload.perihal)
    .bind(payload.no_register_litmas)
    .bind(payload.nomor_surat_permintaan_lapas)
    .bind(payload.jenis_permintaan_litmas_lapas)
    .bind(payload.nama_instansi)
    .bind(payload.kelas_instansi)
    .bind(payload.daerah_instansi)
    .bind(payload.nama_penjamin)
    .bind(payload.alamat_penjamin)
    .bind(updated_by)
    .bind(id)
    .fetch_one(&pool)
    .await;
    
    match result {
        Ok(record) => Ok(Json(record)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update penerimaan_dewasa: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// --- DELETE ---
pub async fn delete_penerimaan_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> StatusCode {
    let klien_id = match sqlx::query_scalar!("SELECT klien_id FROM penerimaan_dewasa WHERE id = $1", id)
        .fetch_optional(&pool).await {
        Ok(Some(id)) => id,
        Ok(None) => return StatusCode::NOT_FOUND,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    if let Err(status) = authorize_user_for_klien(&pool, klien_id, &claims).await {
        return status;
    }
    let result = sqlx::query("DELETE FROM penerimaan_dewasa WHERE id = $1").bind(id).execute(&pool).await;
    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND, // Should be unreachable due to check above, but good practice
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
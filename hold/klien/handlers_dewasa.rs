// src/klien/handlers_dewasa.rs

use crate::auth::model::Claims;
use crate::klien::model_dewasa::{
    CreatePenerimaanDewasa, PenerimaanDewasa, UpdatePenerimaanDewasa,RiwayatHukumDewasa,
    CreateRiwayatHukumDewasa,UpdateRiwayatHukumDewasa,CreateLayananIntegrasiDewasa, LayananIntegrasiDewasa, UpdateLayananIntegrasiDewasa,
    CreateWajibLaporDewasa, WajibLaporDewasa, 
};
use axum::{
    extract::{Extension, Path}, // <-- ADD Header HERE
    http::StatusCode,
    Json,
    http::HeaderMap,
};
use sqlx::PgPool;
use std::env; // To read the KIOSK_API_KEY from the environment


/// A struct to hold authorization context for a request.
pub struct AuthContext {
    pub user_id: i32,
    pub user_role: crate::types::UserRoleEnumEnum,
    pub user_bapas_id: Option<i32>,
    pub klien_bapas_id: i32,
    pub klien_pk_id: i32,
}

impl AuthContext {
    /// Fetches all necessary IDs for an authorization decision for a given klien.
    pub async fn new(pool: &PgPool, klien_id: i32, claims: &Claims) -> Result<Self, StatusCode> {
        let ownership = sqlx::query!(
            "SELECT bapas_id, pk_id FROM klien WHERE id = $1",
            klien_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

        Ok(Self {
            user_id: claims.sub,
            user_role: claims.role.clone(),
            user_bapas_id: claims.bapas_id,
            klien_bapas_id: ownership.bapas_id,
            klien_pk_id: ownership.pk_id,
        })
    }

    /// Checks if the user is in the same Bapas as the client.
    pub fn is_in_same_bapas(&self) -> bool {
        self.user_bapas_id.is_some() && self.user_bapas_id == Some(self.klien_bapas_id)
    }
    
}


/// Helper function to authorize a user for a given klien.
async fn authorize_user_for_klien(
    pool: &PgPool,
    klien_id: i32,
    claims: &Claims,
) -> Result<(), StatusCode> {
    // FIX 1: This query now correctly assigns the record if found, or returns an error if not.
    // The variable `klien_ownership` is now guaranteed to be the record struct.
    let klien_ownership = sqlx::query!(
        "SELECT bapas_id, pk_id FROM klien WHERE id = $1",
        klien_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?; // Note the added semicolon

    let authorized = match claims.role {
        crate::types::UserRoleEnum::SuperAdmin => true,

        crate::types::UserRoleEnum::AdminKanwil => {
            let admin_kanwil_id = match claims.kanwil_id {
                Some(id) => id,
                // FIX 3: If an admin has no kanwil_id, they are not authorized.
                None => return Err(StatusCode::FORBIDDEN),
            };

            // FIX 4: fetch_one + map_err handles all errors. No .ok_or needed.
            let bapas_kanwil_id = sqlx::query_scalar!(
                "SELECT kanwil_id FROM bapas WHERE id = $1",
                klien_ownership.bapas_id
            )
            .fetch_one(pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            admin_kanwil_id == bapas_kanwil_id
        }

        crate::types::UserRoleEnum::AdminBapas => {
            // Using .is_some() and .unwrap() is safer than unwrap_or(-1)
            // It correctly handles the case where bapas_id is None.
            if let Some(user_bapas_id) = claims.bapas_id {
                klien_ownership.bapas_id == user_bapas_id
            } else {
                false // If the admin isn't assigned to a Bapas, they can't access this.
            }
        }
        
        crate::types::UserRoleEnum::Pegawai => klien_ownership.pk_id == claims.sub,
    };

    if authorized {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
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



// in src/klien/handlers_dewasa.rs

// --- Riwayat Hukum Dewasa Handlers ---

// --- Riwayat Hukum Dewasa Handlers ---

/// Creates a `riwayat_hukum_dewasa` record for a specific `klien`.
pub async fn create_riwayat_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateRiwayatHukumDewasa>,
) -> Result<Json<RiwayatHukumDewasa>, StatusCode> {
    authorize_user_for_klien(&pool, klien_id, &claims).await?;
    
    let created_by = claims.sub;

    let result = sqlx::query_as(
        r#"
        INSERT INTO riwayat_hukum_dewasa (
            klien_id, kategori_tindak_pidana, pasal_tindak_pidana,
            tanggal_surat_keputusan_pengadilan, nomor_surat_keputusan_pengadilan,
            pidana_tahun, pidana_bulan, pidana_hari, pertama_ditahan,
            created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING *
        "#,
    )
    .bind(klien_id)
    .bind(payload.kategori_tindak_pidana)
    .bind(payload.pasal_tindak_pidana)
    .bind(payload.tanggal_surat_keputusan_pengadilan)
    .bind(payload.nomor_surat_keputusan_pengadilan)
    .bind(payload.pidana_tahun)
    .bind(payload.pidana_bulan)
    .bind(payload.pidana_hari)
    .bind(payload.pertama_ditahan)
    .bind(created_by)
    .bind(created_by)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => Ok(Json(record)),
        Err(e) => {
            tracing::error!("Failed to create riwayat_hukum_dewasa: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Gets all `riwayat_hukum_dewasa` records for a specific `klien`.
pub async fn get_all_riwayat_hukum_for_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(klien_id): Path<i32>,
) -> Result<Json<Vec<RiwayatHukumDewasa>>, StatusCode> {
    authorize_user_for_klien(&pool, klien_id, &claims).await?;
    
    let records = sqlx::query_as("SELECT * FROM riwayat_hukum_dewasa WHERE klien_id = $1 ORDER BY id DESC")
        .bind(klien_id).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok(Json(records))
}

/// Gets a single `riwayat_hukum_dewasa` record by its own ID.
pub async fn get_riwayat_hukum_by_id(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<RiwayatHukumDewasa>, StatusCode> {
    let record: RiwayatHukumDewasa = sqlx::query_as("SELECT * FROM riwayat_hukum_dewasa WHERE id = $1")
        .bind(id).fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    authorize_user_for_klien(&pool, record.klien_id, &claims).await?;
    
    Ok(Json(record))
}

/// Updates a specific `riwayat_hukum_dewasa` record.
pub async fn update_riwayat_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateRiwayatHukumDewasa>,
) -> Result<Json<RiwayatHukumDewasa>, StatusCode> {
    let klien_id = match sqlx::query_scalar!("SELECT klien_id FROM riwayat_hukum_dewasa WHERE id = $1", id)
        .fetch_optional(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    
    authorize_user_for_klien(&pool, klien_id, &claims).await?;

    let updated_by = claims.sub;
    
    let result = sqlx::query_as(
        r#"
        UPDATE riwayat_hukum_dewasa
        SET
            kategori_tindak_pidana = COALESCE($1, kategori_tindak_pidana),
            pasal_tindak_pidana = COALESCE($2, pasal_tindak_pidana),
            tanggal_surat_keputusan_pengadilan = COALESCE($3, tanggal_surat_keputusan_pengadilan),
            nomor_surat_keputusan_pengadilan = COALESCE($4, nomor_surat_keputusan_pengadilan),
            pidana_tahun = COALESCE($5, pidana_tahun),
            pidana_bulan = COALESCE($6, pidana_bulan),
            pidana_hari = COALESCE($7, pidana_hari),
            pertama_ditahan = COALESCE($8, pertama_ditahan),
            updated_by = $9
        WHERE id = $10
        RETURNING *
        "#,
    )
    .bind(payload.kategori_tindak_pidana)
    .bind(payload.pasal_tindak_pidana)
    .bind(payload.tanggal_surat_keputusan_pengadilan)
    .bind(payload.nomor_surat_keputusan_pengadilan)
    .bind(payload.pidana_tahun)
    .bind(payload.pidana_bulan)
    .bind(payload.pidana_hari)
    .bind(payload.pertama_ditahan)
    .bind(updated_by)
    .bind(id)
    .fetch_one(&pool)
    .await;
        
    match result {
        Ok(record) => Ok(Json(record)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update riwayat_hukum_dewasa: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Deletes a specific `riwayat_hukum_dewasa` record.
pub async fn delete_riwayat_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> StatusCode {
    let klien_id = match sqlx::query_scalar!("SELECT klien_id FROM riwayat_hukum_dewasa WHERE id = $1", id)
        .fetch_optional(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
        
    if let Err(status) = authorize_user_for_klien(&pool, klien_id, &claims).await {
        return status;
    }
    
    let result = sqlx::query("DELETE FROM riwayat_hukum_dewasa WHERE id = $1").bind(id).execute(&pool).await;
    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Creates a `layanan_integrasi_dewasa` record for a specific `klien`.
pub async fn create_layanan_integrasi_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateLayananIntegrasiDewasa>,
) -> Result<Json<LayananIntegrasiDewasa>, StatusCode> {
    // Authorization: Any user with access to the parent klien can create.
    AuthContext::new(&pool, klien_id, &claims).await?;
    
    let created_by = claims.sub;
    let result = sqlx::query_as(
        r#"
        INSERT INTO layanan_integrasi_dewasa (
            klien_id, nomor_sk, tanggal_sk, nomor_register_integrasi,
            masa_bimbingan_awal, masa_bimbingan_akhir, petugas_layanan_id,
            created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(klien_id)
    .bind(payload.nomor_sk)
    .bind(payload.tanggal_sk)
    .bind(payload.nomor_register_integrasi)
    .bind(payload.masa_bimbingan_awal)
    .bind(payload.masa_bimbingan_akhir)
    .bind(payload.petugas_layanan_id)
    .bind(created_by)
    .bind(created_by)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => Ok(Json(record)),
        Err(e) => {
            tracing::error!("Failed to create layanan_integrasi_dewasa: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Gets all `layanan_integrasi_dewasa` records for a specific `klien`.
pub async fn get_all_layanan_integrasi_for_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(klien_id): Path<i32>,
) -> Result<Json<Vec<LayananIntegrasiDewasa>>, StatusCode> {
    AuthContext::new(&pool, klien_id, &claims).await?;
    
    let records = sqlx::query_as("SELECT * FROM layanan_integrasi_dewasa WHERE klien_id = $1 ORDER BY id DESC")
        .bind(klien_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch layanan_integrasi_dewasa: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        
    Ok(Json(records))
}

/// Gets a single `layanan_integrasi_dewasa` record by its own ID.
pub async fn get_layanan_integrasi_by_id(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<LayananIntegrasiDewasa>, StatusCode> {
    let record: LayananIntegrasiDewasa = sqlx::query_as("SELECT * FROM layanan_integrasi_dewasa WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    AuthContext::new(&pool, record.klien_id, &claims).await?;
    
    Ok(Json(record))
}

/// Updates a specific `layanan_integrasi_dewasa` record.
/// Rule: Anyone in the same Bapas can update.
pub async fn update_layanan_integrasi_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateLayananIntegrasiDewasa>,
) -> Result<Json<LayananIntegrasiDewasa>, StatusCode> {
    let klien_id = match sqlx::query_scalar!("SELECT klien_id FROM layanan_integrasi_dewasa WHERE id = $1", id)
        .fetch_optional(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    
    let auth_context = AuthContext::new(&pool, klien_id, &claims).await?;

    // APPLY NEW RULE: Allow if SuperAdmin OR if user is in the same Bapas.
    if auth_context.user_role != crate::types::UserRoleEnum::SuperAdmin && !auth_context.is_in_same_bapas() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let updated_by = claims.sub;
    
    let result = sqlx::query_as(
        r#"
        UPDATE layanan_integrasi_dewasa
        SET
            nomor_sk = COALESCE($1, nomor_sk),
            tanggal_sk = COALESCE($2, tanggal_sk),
            nomor_register_integrasi = COALESCE($3, nomor_register_integrasi),
            masa_bimbingan_awal = COALESCE($4, masa_bimbingan_awal),
            masa_bimbingan_akhir = COALESCE($5, masa_bimbingan_akhir),
            petugas_layanan_id = COALESCE($6, petugas_layanan_id),
            updated_by = $7
        WHERE id = $8
        RETURNING *
        "#,
    )
    .bind(payload.nomor_sk)
    .bind(payload.tanggal_sk)
    .bind(payload.nomor_register_integrasi)
    .bind(payload.masa_bimbingan_awal)
    .bind(payload.masa_bimbingan_akhir)
    .bind(payload.petugas_layanan_id)
    .bind(updated_by)
    .bind(id)
    .fetch_one(&pool)
    .await;
        
    match result {
        Ok(record) => Ok(Json(record)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update layanan_integrasi_dewasa: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Deletes a `layanan_integrasi_dewasa` record.
/// Rule: Only AdminBapas or SuperAdmin can delete.
pub async fn delete_layanan_integrasi_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> StatusCode {
    let klien_id = match sqlx::query_scalar!("SELECT klien_id FROM layanan_integrasi_dewasa WHERE id = $1", id)
        .fetch_optional(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
        
    let auth_context = match AuthContext::new(&pool, klien_id, &claims).await {
        Ok(ctx) => ctx,
        Err(status) => return status,
    };
    
    // APPLY NEW RULE: Only SuperAdmin or AdminBapas can delete.
    if auth_context.user_role != crate::types::UserRoleEnum::SuperAdmin && auth_context.user_role != crate::types::UserRoleEnum::AdminBapas {
        return StatusCode::FORBIDDEN;
    }
    
    let result = sqlx::query("DELETE FROM layanan_integrasi_dewasa WHERE id = $1").bind(id).execute(&pool).await;
    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}


/// Creates a `wajib_lapor_dewasa` from a trusted Kiosk application.
/// This endpoint is public but requires a secret Kiosk API Key.
pub async fn kiosk_wajib_lapor_dewasa(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap, // Get all headers
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateWajibLaporDewasa>,
) -> Result<Json<WajibLaporDewasa>, StatusCode> {

    // Manually extract the X-Api-Key header
    let api_key = headers
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    // Verify the API key against the expected value from environment variables
    let expected_api_key = env::var("KIOSK_API_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if api_key != expected_api_key {
        return Err(StatusCode::UNAUTHORIZED);
    }    
    // 2. Logic for a Kiosk-based report
    let metode_lapor = crate::types::MetodeLaporEnum::SelfService;
    let created_by: Option<i32> = None; // No officer is logged in.

    // 3. Insert the record into the database
     let record = sqlx::query_as(
        r#"
        INSERT INTO wajib_lapor_dewasa (
            klien_id, photo_path, latitude, longitude, metode_lapor, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6) -- $5 is metode_lapor
        RETURNING *
        "#,
    )
    .bind(klien_id)
    .bind(payload.photo_path)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(metode_lapor) // <-- ADD THIS BIND. This will use the variable.
    .bind(created_by)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create kiosk wajib_lapor_dewasa: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(record))
}



pub async fn Online_wajib_lapor_dewasa(
    Extension(pool): Extension<PgPool>,
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateWajibLaporDewasa>,
) -> Result<Json<WajibLaporDewasa>, StatusCode> {
    
    // 1. Fetch the client to check their `online_akses` status.
    let klien = sqlx::query!("SELECT online_akses FROM klien WHERE id = $1", klien_id)
        .fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 2. Authorization: A self-service report is only allowed if the flag is true.
    if !klien.online_akses {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 3. Insert the record with the 'Online' method and NULL creator.
    let metode_lapor = crate::types::MetodeLaporEnum::Online;
    let created_by: Option<i32> = None;

     let record = sqlx::query_as(
        r#"
        INSERT INTO wajib_lapor_dewasa (
            klien_id, photo_path, latitude, longitude, metode_lapor, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6) -- $5 is metode_lapor
        RETURNING *
        "#,
    )
    .bind(klien_id)
    .bind(payload.photo_path)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(metode_lapor) // <-- ADD THIS BIND. This will use the variable.
    .bind(created_by)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create wajib_lapor_dewasa: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(record))
}

pub async fn petugas_wajib_lapor_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>, // This handler requires claims
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateWajibLaporDewasa>,
) -> Result<Json<WajibLaporDewasa>, StatusCode> {
    
    // 1. Authorization: Standard check to see if the officer can access this client.
    authorize_user_for_klien(&pool, klien_id, &claims).await?;
    
    // 2. Insert the record with the 'Petugas' method and the officer's ID.
    let metode_lapor = crate::types::MetodeLaporEnum::Petugas;
    let created_by = Some(claims.sub);

     let record = sqlx::query_as(
        r#"
        INSERT INTO wajib_lapor_dewasa (
            klien_id, photo_path, latitude, longitude, metode_lapor, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6) -- $5 is metode_lapor
        RETURNING *
        "#,
    )
    .bind(klien_id)
    .bind(payload.photo_path)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(metode_lapor) // <-- ADD THIS BIND. This will use the variable.
    .bind(created_by)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create wajib_lapor_dewasa: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(record))
}

// Add this function to the bottom of /src/klien/handlers_dewasa.rs

/// Deletes a specific `wajib_lapor_dewasa` record.
pub async fn delete_wajib_lapor_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>, // BIGSERIAL is i64
) -> StatusCode {
    
    // Fetch the record's parent klien_id for authorization
    let klien_id = match sqlx::query_scalar!("SELECT klien_id FROM wajib_lapor_dewasa WHERE id = $1", id)
        .fetch_optional(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
    
    // Authorize the user against the parent client
    if let Err(status) = authorize_user_for_klien(&pool, klien_id, &claims).await {
        return status;
    }

    // If authorized, proceed with deletion
    let result = sqlx::query("DELETE FROM wajib_lapor_dewasa WHERE id = $1").bind(id).execute(&pool).await;
    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
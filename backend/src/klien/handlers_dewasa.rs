// File: src/klien/handlers_dewasa.rs

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::auth::model::AuthenticatedUser;
use super::model_dewasa::{
    CreatePenerimaanDewasa, PenerimaanDewasa,
    CreateRiwayatHukumDewasa, RiwayatHukumDewasa, UpdateRiwayatHukumDewasa,
    CreateLayananIntegrasiDewasa, LayananIntegrasiDewasa, UpdateLayananIntegrasiDewasa,
    CreateProsesHukumDewasa, ProsesHukumDewasa, UpdateProsesHukumDewasa,
    CreateWajibLapor, WajibLaporDewasa}; // Nanti kita tambah UpdatePenerimaanDewasa

use bcrypt::verify;

















// === PENERIMAAN DEWASA CRUD HANDLERS ===

// --- CREATE ---
// URL: POST /api/klien/:klien_id/penerimaan-dewasa
#[axum::debug_handler]
pub async fn create_penerimaan_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(klien_id): Path<i32>, // Diambil dari URL, sudah diautorisasi oleh middleware
    Json(payload): Json<CreatePenerimaanDewasa>,
) -> Result<Json<PenerimaanDewasa>, StatusCode> {
    
    let new_penerimaan = sqlx::query_as!(
        PenerimaanDewasa,
        r#"
        INSERT INTO penerimaan_dewasa (
            klien_id, tanggal_permintaan_lapas_dewasa, tanggal_surat_tugas_dewasa,
            perihal_dewasa, no_register_litmas_dewasa, nomor_surat_permintaan_lapas_dewasa,
            jenis_permintaan_litmas_lapas_dewasa, nama_instansi_dewasa, kelas_instansi_dewasa,
            daerah_instansi_dewasa, nama_penjamin_dewasa, alamat_penjamin_dewasa,
            kelurahan_penjamin_dewasa, kecamatan_penjamin_dewasa, kota_kabupaten_penjamin_dewasa,
            keterangan, catatan, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $18)
        RETURNING
            id, klien_id, tanggal_permintaan_lapas_dewasa, tanggal_surat_tugas_dewasa, 
            perihal_dewasa, no_register_litmas_dewasa, nomor_surat_permintaan_lapas_dewasa,
            jenis_permintaan_litmas_lapas_dewasa, nama_instansi_dewasa as "nama_instansi_dewasa: _",
            kelas_instansi_dewasa, daerah_instansi_dewasa, nama_penjamin_dewasa,
            alamat_penjamin_dewasa, kelurahan_penjamin_dewasa, kecamatan_penjamin_dewasa,
            kota_kabupaten_penjamin_dewasa, keterangan, catatan, created_at, updated_at,
            created_by, updated_by, deleted_at
        "#,
        klien_id,
        payload.tanggal_permintaan_lapas_dewasa,
        payload.tanggal_surat_tugas_dewasa,
        payload.perihal_dewasa,
        payload.no_register_litmas_dewasa,
        payload.nomor_surat_permintaan_lapas_dewasa,
        payload.jenis_permintaan_litmas_lapas_dewasa,
        payload.nama_instansi_dewasa as _,
        payload.kelas_instansi_dewasa,
        payload.daerah_instansi_dewasa,
        payload.nama_penjamin_dewasa,
        payload.alamat_penjamin_dewasa,
        payload.kelurahan_penjamin_dewasa,
        payload.kecamatan_penjamin_dewasa,
        payload.kota_kabupaten_penjamin_dewasa,
        payload.keterangan,
        payload.catatan,
        user.id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create penerimaan dewasa: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_penerimaan))
}


// --- READ ALL FOR A SPECIFIC KLIEN ---
// URL: GET /api/klien/:klien_id/penerimaan-dewasa
#[axum::debug_handler]
pub async fn get_all_penerimaan_for_klien(
    Extension(pool): Extension<PgPool>,
    Path(klien_id): Path<i32>,
) -> Result<Json<Vec<PenerimaanDewasa>>, StatusCode> {
    
    let penerimaan_list = sqlx::query_as!(
        PenerimaanDewasa,
        r#"
        SELECT
            id, klien_id, tanggal_permintaan_lapas_dewasa, tanggal_surat_tugas_dewasa, 
            perihal_dewasa, no_register_litmas_dewasa, nomor_surat_permintaan_lapas_dewasa,
            jenis_permintaan_litmas_lapas_dewasa, nama_instansi_dewasa as "nama_instansi_dewasa: _",
            kelas_instansi_dewasa, daerah_instansi_dewasa, nama_penjamin_dewasa,
            alamat_penjamin_dewasa, kelurahan_penjamin_dewasa, kecamatan_penjamin_dewasa,
            kota_kabupaten_penjamin_dewasa, keterangan, catatan, created_at, updated_at,
            created_by, updated_by, deleted_at
        FROM penerimaan_dewasa
        WHERE klien_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        klien_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch penerimaan list for klien {}: {}", klien_id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(penerimaan_list))
}


// --- READ ONE BY ITS OWN ID ---
// URL: GET /api/penerimaan-dewasa/:id
#[axum::debug_handler]
pub async fn get_penerimaan_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<PenerimaanDewasa>, StatusCode> {
    
    let penerimaan = sqlx::query_as!(
        PenerimaanDewasa,
        r#"
        SELECT
            id, klien_id, tanggal_permintaan_lapas_dewasa, tanggal_surat_tugas_dewasa, 
            perihal_dewasa, no_register_litmas_dewasa, nomor_surat_permintaan_lapas_dewasa,
            jenis_permintaan_litmas_lapas_dewasa, nama_instansi_dewasa as "nama_instansi_dewasa: _",
            kelas_instansi_dewasa, daerah_instansi_dewasa, nama_penjamin_dewasa,
            alamat_penjamin_dewasa, kelurahan_penjamin_dewasa, kecamatan_penjamin_dewasa,
            kota_kabupaten_penjamin_dewasa, keterangan, catatan, created_at, updated_at,
            created_by, updated_by, deleted_at
        FROM penerimaan_dewasa
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch penerimaan by id {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(penerimaan))
}


// --- UPDATE ---
// URL: PUT /api/penerimaan-dewasa/:id
// Untuk Update, kita akan gunakan struct `CreatePenerimaanDewasa` karena semua fieldnya sudah `Option`
#[axum::debug_handler]
pub async fn update_penerimaan_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(payload): Json<CreatePenerimaanDewasa>, // Reuse the Create struct
) -> Result<Json<PenerimaanDewasa>, StatusCode> {
    
    let updated_penerimaan = sqlx::query_as!(
        PenerimaanDewasa,
        r#"
        UPDATE penerimaan_dewasa
        SET
            tanggal_permintaan_lapas_dewasa = $1,
            tanggal_surat_tugas_dewasa = $2,
            perihal_dewasa = $3,
            no_register_litmas_dewasa = $4,
            -- ... Tambahkan semua field lain dengan COALESCE jika perlu ...
            updated_by = $5
        WHERE id = $6 AND deleted_at IS NULL
        RETURNING
            id, klien_id, tanggal_permintaan_lapas_dewasa, tanggal_surat_tugas_dewasa, 
            perihal_dewasa, no_register_litmas_dewasa, nomor_surat_permintaan_lapas_dewasa,
            jenis_permintaan_litmas_lapas_dewasa, nama_instansi_dewasa as "nama_instansi_dewasa: _",
            kelas_instansi_dewasa, daerah_instansi_dewasa, nama_penjamin_dewasa,
            alamat_penjamin_dewasa, kelurahan_penjamin_dewasa, kecamatan_penjamin_dewasa,
            kota_kabupaten_penjamin_dewasa, keterangan, catatan, created_at, updated_at,
            created_by, updated_by, deleted_at
        "#,
        payload.tanggal_permintaan_lapas_dewasa,
        payload.tanggal_surat_tugas_dewasa,
        payload.perihal_dewasa,
        payload.no_register_litmas_dewasa,
        // ...
        user.id,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update penerimaan {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(updated_penerimaan))
}
// Note: Query UPDATE di atas disingkat. Anda harus menggunakan COALESCE jika struct payload
// Anda untuk update benar-benar memiliki Option<T>, atau set langsung seperti di atas jika
// frontend akan mengirim semua field.

// --- DELETE (SOFT) ---
// URL: DELETE /api/penerimaan-dewasa/:id
#[axum::debug_handler]
pub async fn delete_penerimaan_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> StatusCode {
    
    let result = sqlx::query!(
        "UPDATE penerimaan_dewasa SET deleted_at = NOW(), updated_by = $1 WHERE id = $2",
        user.id,
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(e) => {
            tracing::error!("Failed to delete penerimaan {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}






















// === RIWAYAT HUKUM DEWASA CRUD HANDLERS ===

// --- CREATE ---
// URL: POST /api/klien/:klien_id/riwayat-hukum-dewasa
#[axum::debug_handler]
pub async fn create_riwayat_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateRiwayatHukumDewasa>,
) -> Result<Json<RiwayatHukumDewasa>, StatusCode> {
    
    let new_riwayat = sqlx::query_as!(
        RiwayatHukumDewasa,
        r#"
        INSERT INTO riwayat_hukum_dewasa (
            klien_id, kategori_tindak_pidana_dewasa, pasal_tindak_pidana_dewasa,
            tanggal_surat_keputusan_pengadilan_dewasa, nomor_surat_keputusan_pengadilan_dewasa,
            pidana_tahun_dewasa, pidana_bulan_dewasa, pidana_hari_dewasa, pertama_ditahan_dewasa,
            keterangan, catatan, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $12)
        RETURNING *
        "#,
        klien_id,
        payload.kategori_tindak_pidana_dewasa,
        payload.pasal_tindak_pidana_dewasa,
        payload.tanggal_surat_keputusan_pengadilan_dewasa,
        payload.nomor_surat_keputusan_pengadilan_dewasa,
        payload.pidana_tahun_dewasa,
        payload.pidana_bulan_dewasa,
        payload.pidana_hari_dewasa,
        payload.pertama_ditahan_dewasa,
        payload.keterangan,
        payload.catatan,
        user.id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create riwayat hukum: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_riwayat))
}

// --- READ ALL FOR A SPECIFIC KLIEN ---
// URL: GET /api/klien/:klien_id/riwayat-hukum-dewasa
#[axum::debug_handler]
pub async fn get_all_riwayat_hukum_for_klien(
    Extension(pool): Extension<PgPool>,
    Path(klien_id): Path<i32>,
) -> Result<Json<Vec<RiwayatHukumDewasa>>, StatusCode> {
    
    let list = sqlx::query_as!(
        RiwayatHukumDewasa,
        "SELECT * FROM riwayat_hukum_dewasa WHERE klien_id = $1 AND deleted_at IS NULL ORDER BY tanggal_surat_keputusan_pengadilan_dewasa DESC",
        klien_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch riwayat hukum list: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(list))
}

// --- READ ONE BY ITS OWN ID ---
// URL: GET /api/riwayat-hukum-dewasa/:id
#[axum::debug_handler]
pub async fn get_riwayat_hukum_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<RiwayatHukumDewasa>, StatusCode> {
    
    let riwayat = sqlx::query_as!(
        RiwayatHukumDewasa,
        "SELECT * FROM riwayat_hukum_dewasa WHERE id = $1 AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(riwayat))
}

// --- UPDATE ---
// URL: PUT /api/riwayat-hukum-dewasa/:id
#[axum::debug_handler]
pub async fn update_riwayat_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateRiwayatHukumDewasa>,
) -> Result<Json<RiwayatHukumDewasa>, StatusCode> {
    
    let updated_riwayat = sqlx::query_as!(
        RiwayatHukumDewasa,
        r#"
        UPDATE riwayat_hukum_dewasa
        SET
            kategori_tindak_pidana_dewasa = COALESCE($1, kategori_tindak_pidana_dewasa),
            pasal_tindak_pidana_dewasa = COALESCE($2, pasal_tindak_pidana_dewasa),
            tanggal_surat_keputusan_pengadilan_dewasa = COALESCE($3, tanggal_surat_keputusan_pengadilan_dewasa),
            nomor_surat_keputusan_pengadilan_dewasa = COALESCE($4, nomor_surat_keputusan_pengadilan_dewasa),
            pidana_tahun_dewasa = COALESCE($5, pidana_tahun_dewasa),
            pidana_bulan_dewasa = COALESCE($6, pidana_bulan_dewasa),
            pidana_hari_dewasa = COALESCE($7, pidana_hari_dewasa),
            pertama_ditahan_dewasa = COALESCE($8, pertama_ditahan_dewasa),
            keterangan = COALESCE($9, keterangan),
            catatan = COALESCE($10, catatan),
            updated_by = $11
        WHERE id = $12 AND deleted_at IS NULL
        RETURNING *
        "#,
        payload.kategori_tindak_pidana_dewasa,
        payload.pasal_tindak_pidana_dewasa,
        payload.tanggal_surat_keputusan_pengadilan_dewasa,
        payload.nomor_surat_keputusan_pengadilan_dewasa,
        payload.pidana_tahun_dewasa,
        payload.pidana_bulan_dewasa,
        payload.pidana_hari_dewasa,
        payload.pertama_ditahan_dewasa,
        payload.keterangan,
        payload.catatan,
        user.id,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(updated_riwayat))
}

// --- DELETE (SOFT) ---
// URL: DELETE /api/riwayat-hukum-dewasa/:id
#[axum::debug_handler]
pub async fn delete_riwayat_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> StatusCode {
    
    let result = sqlx::query!(
        "UPDATE riwayat_hukum_dewasa SET deleted_at = NOW(), updated_by = $1 WHERE id = $2",
        user.id,
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}




















// === LAYANAN INTEGRASI DEWASA CRUD HANDLERS ===

// --- CREATE ---
// URL: POST /api/klien/:klien_id/layanan-integrasi-dewasa
#[axum::debug_handler]
pub async fn create_layanan_integrasi_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateLayananIntegrasiDewasa>,
) -> Result<Json<LayananIntegrasiDewasa>, StatusCode> {
    
    let new_layanan = sqlx::query_as!(
        LayananIntegrasiDewasa,
        r#"
        INSERT INTO layanan_integrasi_dewasa (
            klien_id, nomor_sk_dewasa, tanggal_sk_integrasi_dewasa, nomor_register_integrasi_dewasa,
            masa_bimbingan_awal_dewasa, masa_bimbingan_akhir_dewasa, petugas_layanan_id,
            jenis_bimbingan_dewasa, tanggal_surat_pengakhiran_dewasa, nomor_surat_pengakhiran_dewasa,
            pengakhiran_dewasa, keterangan, catatan, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $14)
        RETURNING *
        "#,
        klien_id,
        payload.nomor_sk_dewasa,
        payload.tanggal_sk_integrasi_dewasa,
        payload.nomor_register_integrasi_dewasa,
        payload.masa_bimbingan_awal_dewasa,
        payload.masa_bimbingan_akhir_dewasa,
        payload.petugas_layanan_id,
        payload.jenis_bimbingan_dewasa,
        payload.tanggal_surat_pengakhiran_dewasa,
        payload.nomor_surat_pengakhiran_dewasa,
        payload.pengakhiran_dewasa,
        payload.keterangan,
        payload.catatan,
        user.id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create layanan integrasi: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_layanan))
}

// --- READ ALL FOR A SPECIFIC KLIEN ---
// URL: GET /api/klien/:klien_id/layanan-integrasi-dewasa
#[axum::debug_handler]
pub async fn get_all_layanan_integrasi_for_klien(
    Extension(pool): Extension<PgPool>,
    Path(klien_id): Path<i32>,
) -> Result<Json<Vec<LayananIntegrasiDewasa>>, StatusCode> {
    
    let list = sqlx::query_as!(
        LayananIntegrasiDewasa,
        "SELECT * FROM layanan_integrasi_dewasa WHERE klien_id = $1 AND deleted_at IS NULL ORDER BY masa_bimbingan_awal_dewasa DESC",
        klien_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(list))
}

// --- READ ONE BY ITS OWN ID ---
// URL: GET /api/layanan-integrasi-dewasa/:id
#[axum::debug_handler]
pub async fn get_layanan_integrasi_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<LayananIntegrasiDewasa>, StatusCode> {
    
    let layanan = sqlx::query_as!(
        LayananIntegrasiDewasa,
        "SELECT * FROM layanan_integrasi_dewasa WHERE id = $1 AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(layanan))
}

// --- UPDATE ---
// URL: PUT /api/layanan-integrasi-dewasa/:id
#[axum::debug_handler]
pub async fn update_layanan_integrasi_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateLayananIntegrasiDewasa>,
) -> Result<Json<LayananIntegrasiDewasa>, StatusCode> {
    
    let updated_layanan = sqlx::query_as!(
        LayananIntegrasiDewasa,
        r#"
        UPDATE layanan_integrasi_dewasa
        SET
            nomor_sk_dewasa = COALESCE($1, nomor_sk_dewasa),
            tanggal_sk_integrasi_dewasa = COALESCE($2, tanggal_sk_integrasi_dewasa),
            nomor_register_integrasi_dewasa = COALESCE($3, nomor_register_integrasi_dewasa),
            masa_bimbingan_awal_dewasa = COALESCE($4, masa_bimbingan_awal_dewasa),
            masa_bimbingan_akhir_dewasa = COALESCE($5, masa_bimbingan_akhir_dewasa),
            petugas_layanan_id = COALESCE($6, petugas_layanan_id),
            jenis_bimbingan_dewasa = COALESCE($7, jenis_bimbingan_dewasa),
            tanggal_surat_pengakhiran_dewasa = COALESCE($8, tanggal_surat_pengakhiran_dewasa),
            nomor_surat_pengakhiran_dewasa = COALESCE($9, nomor_surat_pengakhiran_dewasa),
            pengakhiran_dewasa = COALESCE($10, pengakhiran_dewasa),
            keterangan = COALESCE($11, keterangan),
            catatan = COALESCE($12, catatan),
            updated_by = $13
        WHERE id = $14 AND deleted_at IS NULL
        RETURNING *
        "#,
        payload.nomor_sk_dewasa,
        payload.tanggal_sk_integrasi_dewasa,
        payload.nomor_register_integrasi_dewasa,
        payload.masa_bimbingan_awal_dewasa,
        payload.masa_bimbingan_akhir_dewasa,
        payload.petugas_layanan_id,
        payload.jenis_bimbingan_dewasa,
        payload.tanggal_surat_pengakhiran_dewasa,
        payload.nomor_surat_pengakhiran_dewasa,
        payload.pengakhiran_dewasa,
        payload.keterangan,
        payload.catatan,
        user.id,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(updated_layanan))
}

// --- DELETE (SOFT) ---
// URL: DELETE /api/layanan-integrasi-dewasa/:id
#[axum::debug_handler]
pub async fn delete_layanan_integrasi_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> StatusCode {
    
    let result = sqlx::query!(
        "UPDATE layanan_integrasi_dewasa SET deleted_at = NOW(), updated_by = $1 WHERE id = $2",
        user.id,
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
















// === PROSES HUKUM DEWASA CRUD HANDLERS ===

// --- CREATE ---
// URL: POST /api/penerimaan-dewasa/:penerimaan_id/proses-hukum-dewasa
#[axum::debug_handler]
pub async fn create_proses_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(penerimaan_dewasa_id): Path<i32>,
    Json(payload): Json<CreateProsesHukumDewasa>,
) -> Result<Json<ProsesHukumDewasa>, StatusCode> {
    
    let new_proses = sqlx::query_as!(
        ProsesHukumDewasa,
        r#"
        INSERT INTO proses_hukum_dewasa (
            penerimaan_dewasa_id, jenis_proses_hukum_dewasa, nomor_register_proses_hukum_dewasa,
            tanggal_proses_dewasa, keterangan, catatan, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
        RETURNING *
        "#,
        penerimaan_dewasa_id,
        payload.jenis_proses_hukum_dewasa,
        payload.nomor_register_proses_hukum_dewasa,
        payload.tanggal_proses_dewasa,
        payload.keterangan,
        payload.catatan,
        user.id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create proses hukum: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_proses))
}

// --- READ ALL FOR A SPECIFIC PENERIMAAN ---
// URL: GET /api/penerimaan-dewasa/:penerimaan_id/proses-hukum-dewasa
#[axum::debug_handler]
pub async fn get_all_proses_hukum_for_penerimaan(
    Extension(pool): Extension<PgPool>,
    Path(penerimaan_dewasa_id): Path<i32>,
) -> Result<Json<Vec<ProsesHukumDewasa>>, StatusCode> {
    
    let list = sqlx::query_as!(
        ProsesHukumDewasa,
        "SELECT * FROM proses_hukum_dewasa WHERE penerimaan_dewasa_id = $1 AND deleted_at IS NULL ORDER BY tanggal_proses_dewasa DESC",
        penerimaan_dewasa_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(list))
}

// --- READ ONE BY ITS OWN ID ---
// URL: GET /api/proses-hukum-dewasa/:id
#[axum::debug_handler]
pub async fn get_proses_hukum_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i64>, // Ingat, ID adalah BIGINT
) -> Result<Json<ProsesHukumDewasa>, StatusCode> {
    
    let proses = sqlx::query_as!(
        ProsesHukumDewasa,
        "SELECT * FROM proses_hukum_dewasa WHERE id = $1 AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(proses))
}

// --- UPDATE ---
// URL: PUT /api/proses-hukum-dewasa/:id
#[axum::debug_handler]
pub async fn update_proses_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateProsesHukumDewasa>,
) -> Result<Json<ProsesHukumDewasa>, StatusCode> {
    
    let updated_proses = sqlx::query_as!(
        ProsesHukumDewasa,
        r#"
        UPDATE proses_hukum_dewasa
        SET
            jenis_proses_hukum_dewasa = COALESCE($1, jenis_proses_hukum_dewasa),
            nomor_register_proses_hukum_dewasa = COALESCE($2, nomor_register_proses_hukum_dewasa),
            tanggal_proses_dewasa = COALESCE($3, tanggal_proses_dewasa),
            keterangan = COALESCE($4, keterangan),
            catatan = COALESCE($5, catatan),
            updated_by = $6
        WHERE id = $7 AND deleted_at IS NULL
        RETURNING *
        "#,
        payload.jenis_proses_hukum_dewasa,
        payload.nomor_register_proses_hukum_dewasa,
        payload.tanggal_proses_dewasa,
        payload.keterangan,
        payload.catatan,
        user.id,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(updated_proses))
}

// --- DELETE (SOFT) ---
// URL: DELETE /api/proses-hukum-dewasa/:id
#[axum::debug_handler]
pub async fn delete_proses_hukum_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i64>,
) -> StatusCode {
    
    let result = sqlx::query!(
        "UPDATE proses_hukum_dewasa SET deleted_at = NOW(), updated_by = $1 WHERE id = $2",
        user.id,
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}










// === WAJIB LAPOR DEWASA HANDLERS ===

// --- CREATE (PETUGAS) ---
// URL: POST /api/petugas/klien/:klien_id/wajib-lapor-dewasa
pub async fn petugas_wajib_lapor_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateWajibLapor>,
) -> StatusCode {
     let result = sqlx::query!(
        r#"
        INSERT INTO wajib_lapor_dewasa 
            (klien_id, metode_lapor_dewasa, created_by, photo_path_dewasa, latitude_dewasa, longitude_dewasa)
        VALUES ($1, 'Petugas', $2, $3, $4, $5)
        "#,
        klien_id,
        user.id,
        payload.photo_path_dewasa,
        payload.latitude_dewasa,
        payload.longitude_dewasa
    )
    .execute(&pool)
    .await;
    
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            tracing::error!("Failed to create wajib lapor (petugas): {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

// --- CREATE (KIOSK) ---
// URL: POST /api/kiosk/klien/:klien_id/wajib-lapor-dewasa
pub async fn kiosk_wajib_lapor_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>, // User "kiosk"
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateWajibLapor>,
) -> StatusCode {
    let result = sqlx::query!(
        r#"
        INSERT INTO wajib_lapor_dewasa 
            (klien_id, metode_lapor_dewasa, created_by, photo_path_dewasa, latitude_dewasa, longitude_dewasa)
        VALUES ($1, 'Self-Service', $2, $3, $4, $5)
        "#,
        klien_id,
        user.id, // ID dari user "kiosk"
        payload.photo_path_dewasa,
        payload.latitude_dewasa,
        payload.longitude_dewasa
    )
    .execute(&pool)
    .await;
    
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            tracing::error!("Failed to create wajib lapor (kiosk): {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

// --- CREATE (MANDIRI) ---
// URL: POST /api/mandiri/klien/:klien_id/wajib-lapor-dewasa
pub async fn mandiri_wajib_lapor_dewasa(
    Extension(pool): Extension<PgPool>,
    Path(klien_id): Path<i32>,
    Json(payload): Json<CreateWajibLapor>,
) -> StatusCode {
    // 1. Fetch klien untuk memeriksa akses online dan hash PIN
    let klien_data = match sqlx::query!("SELECT online_akses_klien, pin_klien_hash FROM klien WHERE id = $1", klien_id)
        .fetch_optional(&pool)
        .await {
            Ok(Some(data)) => data,
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };

    // 2. Otorisasi Bisnis: Cek akses online
    if !klien_data.online_akses_klien {
        return StatusCode::FORBIDDEN; // Akses online tidak diizinkan
    }

    // 3. Verifikasi PIN
    let pin_hash: String = match klien_data.pin_klien_hash { // [FIX] Beri tipe eksplisit
    Some(hash) => hash,
    None => return StatusCode::FORBIDDEN,
};
    let pin_from_payload = match payload.pin {
        Some(pin) => pin,
        None => return StatusCode::UNAUTHORIZED, // PIN tidak disediakan
    };

    if !verify(&pin_from_payload, &pin_hash).unwrap_or(false) {
        return StatusCode::UNAUTHORIZED; // PIN salah
    }

    // 4. Jika semua verifikasi lolos, INSERT data
    let result = sqlx::query!(
        r#"
        INSERT INTO wajib_lapor_dewasa 
            (klien_id, metode_lapor_dewasa, photo_path_dewasa, latitude_dewasa, longitude_dewasa)
        VALUES ($1, 'Online', $2, $3, $4)
        "#,
        klien_id,
        payload.photo_path_dewasa,
        payload.latitude_dewasa,
        payload.longitude_dewasa
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}


// --- READ ALL FOR A KLIEN ---
// URL: GET /api/klien/:klien_id/wajib-lapor-dewasa
// --- READ ALL FOR A KLIEN ---
// URL: GET /api/klien/:klien_id/wajib-lapor-dewasa
pub async fn get_all_wajib_lapor_for_klien(
    Extension(pool): Extension<PgPool>,
    Path(klien_id): Path<i32>,
) -> Result<Json<Vec<WajibLaporDewasa>>, StatusCode> {
    let list = sqlx::query_as!(
        WajibLaporDewasa,
        r#"
        SELECT 
            id,
            klien_id,
            photo_path_dewasa,
            latitude_dewasa,
            longitude_dewasa,
            metode_lapor_dewasa AS "metode_lapor_dewasa: _",
            created_by,
            deleted_at,
            created_at -- [FIX] Kolom ini harus ada dan tidak ada koma setelahnya
        FROM wajib_lapor_dewasa 
        WHERE klien_id = $1 AND deleted_at IS NULL 
        ORDER BY created_at DESC
        "#,
        klien_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch wajib lapor list: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(list))
}


// --- DELETE ---
// URL: DELETE /api/wajib-lapor-dewasa/:id
pub async fn delete_wajib_lapor_dewasa(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i64>,
) -> StatusCode {
    // Otorisasi sudah ditangani oleh middleware `authorize_wajib_lapor_access` yang akan kita buat.
    // Untuk sekarang, kita bisa tambahkan cek role sederhana.
    
    let result = sqlx::query!(
        "UPDATE wajib_lapor_dewasa SET deleted_at = NOW() WHERE id = $1",
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
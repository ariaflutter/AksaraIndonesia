// src/klien/model_dewasa.rs

use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::types::{MetodeLaporEnum, NamaInstansiEnum};

// --- Penerimaan Dewasa Models ---

#[derive(Debug, Serialize, FromRow)]
pub struct PenerimaanDewasa {
    pub id: i32,
    pub klien_id: i32,
    pub tanggal_permintaan_lapas_dewasa: Option<NaiveDate>,
    pub tanggal_surat_tugas_dewasa: Option<NaiveDate>,
    pub perihal_dewasa: Option<String>,
    pub no_register_litmas_dewasa: Option<String>,
    pub nomor_surat_permintaan_lapas_dewasa: Option<String>,
    pub jenis_permintaan_litmas_lapas_dewasa: Option<String>,
    pub nama_instansi_dewasa: Option<NamaInstansiEnum>,
    pub kelas_instansi_dewasa: Option<String>,
    pub daerah_instansi_dewasa: Option<String>,
    pub nama_penjamin_dewasa: Option<String>,
    pub alamat_penjamin_dewasa: Option<String>,
    pub kelurahan_penjamin_dewasa: Option<String>, // [ADD] Missing field
    pub kecamatan_penjamin_dewasa: Option<String>, // [ADD] Missing field
    pub kota_kabupaten_penjamin_dewasa: Option<String>, // [ADD] Missing field
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePenerimaanDewasa {
    // klien_id akan diambil dari URL, bukan dari body
    pub tanggal_permintaan_lapas_dewasa: Option<NaiveDate>,
    pub tanggal_surat_tugas_dewasa: Option<NaiveDate>,
    pub perihal_dewasa: Option<String>,
    pub no_register_litmas_dewasa: Option<String>,
    pub nomor_surat_permintaan_lapas_dewasa: Option<String>,
    pub jenis_permintaan_litmas_lapas_dewasa: Option<String>,
    pub nama_instansi_dewasa: Option<NamaInstansiEnum>,
    pub kelas_instansi_dewasa: Option<String>,
    pub daerah_instansi_dewasa: Option<String>,
    pub nama_penjamin_dewasa: Option<String>,
    pub alamat_penjamin_dewasa: Option<String>,
    pub kelurahan_penjamin_dewasa: Option<String>, // [ADD] Missing field
    pub kecamatan_penjamin_dewasa: Option<String>, // [ADD] Missing field
    pub kota_kabupaten_penjamin_dewasa: Option<String>, // [ADD] Missing field
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

// UpdatePenerimaanDewasa menggunakan struct yang sama dengan Create untuk saat ini
// karena semua field sudah optional. Jika perlu validasi berbeda, bisa dipisah.

// --- Riwayat Hukum Dewasa Models ---

#[derive(Debug, Deserialize)]
pub struct CreateRiwayatHukumDewasa {
    // klien_id akan diambil dari URL
    pub kategori_tindak_pidana_dewasa: Option<String>,
    pub pasal_tindak_pidana_dewasa: Option<String>,
    pub tanggal_surat_keputusan_pengadilan_dewasa: Option<NaiveDate>,
    pub nomor_surat_keputusan_pengadilan_dewasa: Option<String>,
    pub pidana_tahun_dewasa: Option<i32>,
    pub pidana_bulan_dewasa: Option<i32>,
    pub pidana_hari_dewasa: Option<i32>,
    pub pertama_ditahan_dewasa: Option<NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

// [ADD] Struct untuk UPDATE
#[derive(Debug, Deserialize)]
pub struct UpdateRiwayatHukumDewasa {
    pub kategori_tindak_pidana_dewasa: Option<String>,
    pub pasal_tindak_pidana_dewasa: Option<String>,
    pub tanggal_surat_keputusan_pengadilan_dewasa: Option<NaiveDate>,
    pub nomor_surat_keputusan_pengadilan_dewasa: Option<String>,
    pub pidana_tahun_dewasa: Option<i32>,
    pub pidana_bulan_dewasa: Option<i32>,
    pub pidana_hari_dewasa: Option<i32>,
    pub pertama_ditahan_dewasa: Option<NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

// Create/Update untuk Riwayat Hukum Dewasa sudah cocok dengan skema.

// --- Layanan Integrasi Dewasa Models ---

// [FIX] Model ini disesuaikan agar 100% cocok dengan skema DB
#[derive(Debug, Serialize, FromRow)]
pub struct LayananIntegrasiDewasa {
    pub id: i32,
    pub klien_id: i32,
    pub nomor_sk_dewasa: Option<String>,
    pub tanggal_sk_integrasi_dewasa: Option<NaiveDate>,
    pub nomor_register_integrasi_dewasa: Option<String>,
    pub masa_bimbingan_awal_dewasa: Option<NaiveDate>,
    pub masa_bimbingan_akhir_dewasa: Option<NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
    pub jenis_bimbingan_dewasa: Option<String>,
    pub tanggal_surat_pengakhiran_dewasa: Option<NaiveDate>,
    pub nomor_surat_pengakhiran_dewasa: Option<String>,
    pub pengakhiran_dewasa: Option<bool>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

// [FIX] Struct Create disesuaikan dengan skema DB
#[derive(Debug, Deserialize)]
pub struct CreateLayananIntegrasiDewasa {
    pub nomor_sk_dewasa: Option<String>,
    pub tanggal_sk_integrasi_dewasa: Option<NaiveDate>,
    pub nomor_register_integrasi_dewasa: Option<String>,
    pub masa_bimbingan_awal_dewasa: Option<NaiveDate>,
    pub masa_bimbingan_akhir_dewasa: Option<NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
    pub jenis_bimbingan_dewasa: Option<String>,
    pub tanggal_surat_pengakhiran_dewasa: Option<NaiveDate>,
    pub nomor_surat_pengakhiran_dewasa: Option<String>,
    pub pengakhiran_dewasa: Option<bool>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

// [ADD] Struct untuk UPDATE
#[derive(Debug, Deserialize)]
pub struct UpdateLayananIntegrasiDewasa {
    pub nomor_sk_dewasa: Option<String>,
    pub tanggal_sk_integrasi_dewasa: Option<NaiveDate>,
    pub nomor_register_integrasi_dewasa: Option<String>,
    pub masa_bimbingan_awal_dewasa: Option<NaiveDate>,
    pub masa_bimbingan_akhir_dewasa: Option<NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
    pub jenis_bimbingan_dewasa: Option<String>,
    pub tanggal_surat_pengakhiran_dewasa: Option<NaiveDate>,
    pub nomor_surat_pengakhiran_dewasa: Option<String>,
    pub pengakhiran_dewasa: Option<bool>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}



// --- Proses Hukum Dewasa Models ---

#[derive(Debug, Serialize, FromRow)]
pub struct ProsesHukumDewasa {
    pub id: i64, // BIGSERIAL -> i64
    pub klien_id: Option<i32>, // Diisi oleh trigger, tapi bisa NULL di skema
    pub penerimaan_dewasa_id: i32,
    pub jenis_proses_hukum_dewasa: Option<String>,
    pub nomor_register_proses_hukum_dewasa: Option<String>,
    pub tanggal_proses_dewasa: Option<NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProsesHukumDewasa {
    // penerimaan_dewasa_id akan diambil dari URL
    pub jenis_proses_hukum_dewasa: Option<String>,
    pub nomor_register_proses_hukum_dewasa: Option<String>,
    pub tanggal_proses_dewasa: Option<NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProsesHukumDewasa {
    pub jenis_proses_hukum_dewasa: Option<String>,
    pub nomor_register_proses_hukum_dewasa: Option<String>,
    pub tanggal_proses_dewasa: Option<NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}




// Merepresentasikan data yang dibaca dari DB
#[derive(Debug, Serialize, FromRow)]
pub struct WajibLaporDewasa {
    pub id: i64,
    pub klien_id: i32,
    pub waktu_lapor_dewasa: DateTime<Utc>,
    pub photo_path_dewasa: Option<String>,
    pub latitude_dewasa: Option<Decimal>,
    pub longitude_dewasa: Option<Decimal>,
    pub metode_lapor_dewasa: MetodeLaporEnum,
    pub created_by: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// Untuk endpoint Petugas
#[derive(Debug, Deserialize)]
pub struct PetugasCreateWajibLapor {
    pub photo_path_dewasa: String,
    pub latitude_dewasa: Decimal,
    pub longitude_dewasa: Decimal,
}

// Untuk endpoint Kiosk
#[derive(Debug, Deserialize)]
pub struct KioskCreateWajibLapor {
    pub photo_path_dewasa: String,
    pub latitude_dewasa: Decimal,
    pub longitude_dewasa: Decimal,
}

// Untuk endpoint Mandiri (Online)
#[derive(Debug, Deserialize)]
pub struct MandiriCreateWajibLapor {
    pub photo_path_dewasa: String,
    pub latitude_dewasa: Decimal,
    pub longitude_dewasa: Decimal,
}










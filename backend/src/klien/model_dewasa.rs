// AUTO-GENERATED MODELS FROM DB SCHEMA

use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::types::{MetodeLaporEnum, NamaInstansiEnum};
use std::fmt::Debug;
// === LayananIntegrasiDewasa Models ===

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct LayananIntegrasiDewasa {
    pub id: i32,
    pub klien_id: i32,
    pub nomor_sk_dewasa: Option<String>,
    pub tanggal_sk_integrasi_dewasa: Option<chrono::NaiveDate>,
    pub nomor_register_integrasi_dewasa: Option<String>,
    pub masa_bimbingan_awal_dewasa: Option<chrono::NaiveDate>,
    pub masa_bimbingan_akhir_dewasa: Option<chrono::NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
    pub jenis_bimbingan_dewasa: Option<String>,
    pub tanggal_surat_pengakhiran_dewasa: Option<chrono::NaiveDate>,
    pub nomor_surat_pengakhiran_dewasa: Option<String>,
    pub pengakhiran_dewasa: Option<bool>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateLayananIntegrasiDewasa {
    pub klien_id: i32,
    pub nomor_sk_dewasa: Option<String>,
    pub tanggal_sk_integrasi_dewasa: Option<chrono::NaiveDate>,
    pub nomor_register_integrasi_dewasa: Option<String>,
    pub masa_bimbingan_awal_dewasa: Option<chrono::NaiveDate>,
    pub masa_bimbingan_akhir_dewasa: Option<chrono::NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
    pub jenis_bimbingan_dewasa: Option<String>,
    pub tanggal_surat_pengakhiran_dewasa: Option<chrono::NaiveDate>,
    pub nomor_surat_pengakhiran_dewasa: Option<String>,
    pub pengakhiran_dewasa: Option<bool>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateLayananIntegrasiDewasa {
    pub klien_id: Option<i32>,
    pub nomor_sk_dewasa: Option<String>,
    pub tanggal_sk_integrasi_dewasa: Option<chrono::NaiveDate>,
    pub nomor_register_integrasi_dewasa: Option<String>,
    pub masa_bimbingan_awal_dewasa: Option<chrono::NaiveDate>,
    pub masa_bimbingan_akhir_dewasa: Option<chrono::NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
    pub jenis_bimbingan_dewasa: Option<String>,
    pub tanggal_surat_pengakhiran_dewasa: Option<chrono::NaiveDate>,
    pub nomor_surat_pengakhiran_dewasa: Option<String>,
    pub pengakhiran_dewasa: Option<bool>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// === PenerimaanDewasa Models ===

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct PenerimaanDewasa {
    pub id: i32,
    pub klien_id: i32,
    pub tanggal_permintaan_lapas_dewasa: Option<chrono::NaiveDate>,
    pub tanggal_surat_tugas_dewasa: Option<chrono::NaiveDate>,
    pub perihal_dewasa: Option<String>,
    pub no_register_litmas_dewasa: Option<String>,
    pub nomor_surat_permintaan_lapas_dewasa: Option<String>,
    pub jenis_permintaan_litmas_lapas_dewasa: Option<String>,
    pub nama_instansi_dewasa: Option<NamaInstansiEnum>,
    pub kelas_instansi_dewasa: Option<String>,
    pub daerah_instansi_dewasa: Option<String>,
    pub nama_penjamin_dewasa: Option<String>,
    pub alamat_penjamin_dewasa: Option<String>,
    pub kelurahan_penjamin_dewasa: Option<String>,
    pub kecamatan_penjamin_dewasa: Option<String>,
    pub kota_kabupaten_penjamin_dewasa: Option<String>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreatePenerimaanDewasa {
    pub klien_id: i32,
    pub tanggal_permintaan_lapas_dewasa: Option<chrono::NaiveDate>,
    pub tanggal_surat_tugas_dewasa: Option<chrono::NaiveDate>,
    pub perihal_dewasa: Option<String>,
    pub no_register_litmas_dewasa: Option<String>,
    pub nomor_surat_permintaan_lapas_dewasa: Option<String>,
    pub jenis_permintaan_litmas_lapas_dewasa: Option<String>,
    pub nama_instansi_dewasa: Option<NamaInstansiEnum>,
    pub kelas_instansi_dewasa: Option<String>,
    pub daerah_instansi_dewasa: Option<String>,
    pub nama_penjamin_dewasa: Option<String>,
    pub alamat_penjamin_dewasa: Option<String>,
    pub kelurahan_penjamin_dewasa: Option<String>,
    pub kecamatan_penjamin_dewasa: Option<String>,
    pub kota_kabupaten_penjamin_dewasa: Option<String>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdatePenerimaanDewasa {
    pub klien_id: Option<i32>,
    pub tanggal_permintaan_lapas_dewasa: Option<chrono::NaiveDate>,
    pub tanggal_surat_tugas_dewasa: Option<chrono::NaiveDate>,
    pub perihal_dewasa: Option<String>,
    pub no_register_litmas_dewasa: Option<String>,
    pub nomor_surat_permintaan_lapas_dewasa: Option<String>,
    pub jenis_permintaan_litmas_lapas_dewasa: Option<String>,
    pub nama_instansi_dewasa: Option<NamaInstansiEnum>,
    pub kelas_instansi_dewasa: Option<String>,
    pub daerah_instansi_dewasa: Option<String>,
    pub nama_penjamin_dewasa: Option<String>,
    pub alamat_penjamin_dewasa: Option<String>,
    pub kelurahan_penjamin_dewasa: Option<String>,
    pub kecamatan_penjamin_dewasa: Option<String>,
    pub kota_kabupaten_penjamin_dewasa: Option<String>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// === ProsesHukumDewasa Models ===

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ProsesHukumDewasa {
    pub id: i64,
    pub klien_id: Option<i32>,
    pub penerimaan_dewasa_id: i32,
    pub jenis_proses_hukum_dewasa: Option<String>,
    pub nomor_register_proses_hukum_dewasa: Option<String>,
    pub tanggal_proses_dewasa: Option<chrono::NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateProsesHukumDewasa {
    pub penerimaan_dewasa_id: i32,
    pub jenis_proses_hukum_dewasa: Option<String>,
    pub nomor_register_proses_hukum_dewasa: Option<String>,
    pub tanggal_proses_dewasa: Option<chrono::NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateProsesHukumDewasa {
    pub penerimaan_dewasa_id: Option<i32>,
    pub jenis_proses_hukum_dewasa: Option<String>,
    pub nomor_register_proses_hukum_dewasa: Option<String>,
    pub tanggal_proses_dewasa: Option<chrono::NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// === RiwayatHukumDewasa Models ===

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct RiwayatHukumDewasa {
    pub id: i32,
    pub klien_id: i32,
    pub kategori_tindak_pidana_dewasa: Option<String>,
    pub pasal_tindak_pidana_dewasa: Option<String>,
    pub tanggal_surat_keputusan_pengadilan_dewasa: Option<chrono::NaiveDate>,
    pub nomor_surat_keputusan_pengadilan_dewasa: Option<String>,
    pub pidana_tahun_dewasa: Option<i32>,
    pub pidana_bulan_dewasa: Option<i32>,
    pub pidana_hari_dewasa: Option<i32>,
    pub pertama_ditahan_dewasa: Option<chrono::NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateRiwayatHukumDewasa {
    pub klien_id: i32,
    pub kategori_tindak_pidana_dewasa: Option<String>,
    pub pasal_tindak_pidana_dewasa: Option<String>,
    pub tanggal_surat_keputusan_pengadilan_dewasa: Option<chrono::NaiveDate>,
    pub nomor_surat_keputusan_pengadilan_dewasa: Option<String>,
    pub pidana_tahun_dewasa: Option<i32>,
    pub pidana_bulan_dewasa: Option<i32>,
    pub pidana_hari_dewasa: Option<i32>,
    pub pertama_ditahan_dewasa: Option<chrono::NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateRiwayatHukumDewasa {
    pub klien_id: Option<i32>,
    pub kategori_tindak_pidana_dewasa: Option<String>,
    pub pasal_tindak_pidana_dewasa: Option<String>,
    pub tanggal_surat_keputusan_pengadilan_dewasa: Option<chrono::NaiveDate>,
    pub nomor_surat_keputusan_pengadilan_dewasa: Option<String>,
    pub pidana_tahun_dewasa: Option<i32>,
    pub pidana_bulan_dewasa: Option<i32>,
    pub pidana_hari_dewasa: Option<i32>,
    pub pertama_ditahan_dewasa: Option<chrono::NaiveDate>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct WajibLaporDewasa {
    pub id: i64,
    pub klien_id: i32,
    // TIDAK ADA LAGI waktu_lapor_dewasa
    pub photo_path_dewasa: Option<String>,
    pub latitude_dewasa: Option<rust_decimal::Decimal>,
    pub longitude_dewasa: Option<rust_decimal::Decimal>,
    pub metode_lapor_dewasa: MetodeLaporEnum,
    pub created_by: Option<i32>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>, // [PENTING] Kita akan pakai ini
}

// Struct ini juga LEBIH SEDERHANA
#[derive(Debug, serde::Deserialize)]
pub struct CreateWajibLapor {
    // TIDAK ADA LAGI waktu_lapor_dewasa
    pub photo_path_dewasa: String,
    pub latitude_dewasa: rust_decimal::Decimal,
    pub longitude_dewasa: rust_decimal::Decimal,
    pub pin: Option<String>,
}

// 3. Struct untuk MENGUPDATE data (jika diperlukan di masa depan).
//    Untuk saat ini, kita tidak punya endpoint UPDATE untuk wajib lapor,
//    tapi ada baiknya didefinisikan untuk kelengkapan.
#[derive(Debug, serde::Deserialize)]
pub struct UpdateWajibLapor {
    pub photo_path_dewasa: Option<String>,
    pub latitude_dewasa: Option<rust_decimal::Decimal>,
    pub longitude_dewasa: Option<rust_decimal::Decimal>,
}






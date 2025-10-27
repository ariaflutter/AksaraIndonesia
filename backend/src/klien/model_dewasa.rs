use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::types::{NamaInstansi, MetodeLapor}; // Make sure MetodeLapor is here
use rust_decimal::Decimal; // <-- ADD THIS IMPORT

// This struct represents a `penerimaan_dewasa` record as it is READ from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PenerimaanDewasa {
    pub id: i32,
    pub klien_id: i32,
    pub tanggal_permintaan_lapas: Option<NaiveDate>,
    pub tanggal_surat_tugas: Option<NaiveDate>,
    pub perihal: Option<String>,
    pub no_register_litmas: Option<String>,
    pub nomor_surat_permintaan_lapas: Option<String>,
    pub jenis_permintaan_litmas_lapas: Option<String>,
    pub nama_instansi: Option<NamaInstansi>,
    pub kelas_instansi: Option<String>,
    pub daerah_instansi: Option<String>,
    pub nama_penjamin: Option<String>,
    pub alamat_penjamin: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

// This struct represents the JSON payload for CREATING a new `penerimaan_dewasa`.
#[derive(Debug, Deserialize)]
pub struct CreatePenerimaanDewasa {
    pub tanggal_permintaan_lapas: Option<NaiveDate>,
    pub tanggal_surat_tugas: Option<NaiveDate>,
    pub perihal: Option<String>,
    pub no_register_litmas: Option<String>,
    pub nomor_surat_permintaan_lapas: Option<String>,
    pub jenis_permintaan_litmas_lapas: Option<String>,
    pub nama_instansi: Option<NamaInstansi>,
    pub kelas_instansi: Option<String>,
    pub daerah_instansi: Option<String>,
    pub nama_penjamin: Option<String>,
    pub alamat_penjamin: Option<String>,
}

// This struct represents the JSON payload for UPDATING a `penerimaan_dewasa`.
// All fields are optional to allow for partial updates.
#[derive(Debug, Deserialize)]
pub struct UpdatePenerimaanDewasa {
    pub tanggal_permintaan_lapas: Option<NaiveDate>,
    pub tanggal_surat_tugas: Option<NaiveDate>,
    pub perihal: Option<String>,
    pub no_register_litmas: Option<String>,
    pub nomor_surat_permintaan_lapas: Option<String>,
    pub jenis_permintaan_litmas_lapas: Option<String>,
    pub nama_instansi: Option<NamaInstansi>,
    pub kelas_instansi: Option<String>,
    pub daerah_instansi: Option<String>,
    pub nama_penjamin: Option<String>,
    pub alamat_penjamin: Option<String>,
}


// --- Riwayat Hukum Dewasa Models ---

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RiwayatHukumDewasa {
    pub id: i32,
    pub klien_id: i32, // Correctly linked to klien
    pub kategori_tindak_pidana: Option<String>,
    pub pasal_tindak_pidana: Option<String>,
    pub tanggal_surat_keputusan_pengadilan: Option<NaiveDate>,
    pub nomor_surat_keputusan_pengadilan: Option<String>,
    pub pidana_tahun: Option<i32>,
    pub pidana_bulan: Option<i32>,
    pub pidana_hari: Option<i32>,
    pub pertama_ditahan: Option<NaiveDate>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRiwayatHukumDewasa {
    pub kategori_tindak_pidana: Option<String>,
    pub pasal_tindak_pidana: Option<String>,
    pub tanggal_surat_keputusan_pengadilan: Option<NaiveDate>,
    pub nomor_surat_keputusan_pengadilan: Option<String>,
    pub pidana_tahun: Option<i32>,
    pub pidana_bulan: Option<i32>,
    pub pidana_hari: Option<i32>,
    pub pertama_ditahan: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRiwayatHukumDewasa {
    pub kategori_tindak_pidana: Option<String>,
    pub pasal_tindak_pidana: Option<String>,
    pub tanggal_surat_keputusan_pengadilan: Option<NaiveDate>,
    pub nomor_surat_keputusan_pengadilan: Option<String>,
    pub pidana_tahun: Option<i32>,
    pub pidana_bulan: Option<i32>,
    pub pidana_hari: Option<i32>,
    pub pertama_ditahan: Option<NaiveDate>,
}

// --- Layanan Integrasi Dewasa Models ---

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LayananIntegrasiDewasa {
    pub id: i32,
    pub klien_id: i32, // Corrected to link to Klien
    pub nomor_sk: Option<String>,
    pub tanggal_sk: Option<NaiveDate>,
    pub nomor_register_integrasi: Option<String>,
    pub masa_bimbingan_awal: Option<NaiveDate>,
    pub masa_bimbingan_akhir: Option<NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLayananIntegrasiDewasa {
    pub nomor_sk: Option<String>,
    pub tanggal_sk: Option<NaiveDate>,
    pub nomor_register_integrasi: Option<String>,
    pub masa_bimbingan_awal: Option<NaiveDate>,
    pub masa_bimbingan_akhir: Option<NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLayananIntegrasiDewasa { // <-- The UPDATE struct you requested
    pub nomor_sk: Option<String>,
    pub tanggal_sk: Option<NaiveDate>,
    pub nomor_register_integrasi: Option<String>,
    pub masa_bimbingan_awal: Option<NaiveDate>,
    pub masa_bimbingan_akhir: Option<NaiveDate>,
    pub petugas_layanan_id: Option<i32>,
}


// --- Wajib Lapor Dewasa Models ---

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WajibLaporDewasa {
    pub id: i64,
    pub klien_id: i32,
    pub waktu_lapor: chrono::DateTime<chrono::Utc>,
    pub photo_path: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub metode_lapor: MetodeLapor, // <-- ADD THIS
    pub created_at: chrono::DateTime<chrono::Utc>,

}

#[derive(Debug, Deserialize)]
pub struct CreateWajibLaporDewasa {
    pub photo_path: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    #[serde(default = "default_metode_lapor")]
    pub _metode_lapor: MetodeLapor, // <-- This is the unused field
}
// A helper function for serde to provide the default value.
fn default_metode_lapor() -> MetodeLapor {
    MetodeLapor::Petugas
}
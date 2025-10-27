use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::types::NamaInstansi; 

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
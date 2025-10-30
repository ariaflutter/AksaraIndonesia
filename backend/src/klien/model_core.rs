// src/klien/model_core.rs

// [FIX] Import semua enum yang dibutuhkan dari crate::types
use crate::types::{
    TipeKlienEnum, 
    TingkatPendidikanEnum, 
    JenisPekerjaanEnum, 
    JenisKelaminEnum, 
    KewarganegaraanEnum
};
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Merepresentasikan satu baris dari tabel 'klien'
#[derive(Debug, Serialize, FromRow)]
pub struct Klien {
    pub id: i32,
    pub tipe_klien: TipeKlienEnum,
    pub nama_klien: String,
    pub alamat_klien: Option<String>,
    pub tempat_lahir_klien: Option<String>,
    pub tanggal_lahir_klien: Option<NaiveDate>,
    pub jenis_kelamin_klien: Option<JenisKelaminEnum>,
    pub agama_klien: Option<String>,
    pub pekerjaan_klien: Option<JenisPekerjaanEnum>,
    pub pendidikan_terakhir_klien: Option<TingkatPendidikanEnum>,
    pub bapas_id: i32,
    pub pk_id: i32,
    pub kanwil_id: Option<i32>,
    pub online_akses_klien: bool,
    pub pengulangan_klien: bool,
    pub kewarganegaraan_klien: Option<KewarganegaraanEnum>,
    pub negara_asal_klien: Option<String>,
    pub suku_klien: Option<String>,
    pub keterangan_klien: Option<String>,
    pub catatan_klien: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

// Data yang dibutuhkan untuk MEMBUAT Klien baru
#[derive(Debug, Deserialize)]
pub struct CreateKlien {
    pub tipe_klien: TipeKlienEnum,
    pub nama_klien: String,
    pub alamat_klien: Option<String>,
    pub tempat_lahir_klien: Option<String>,
    pub tanggal_lahir_klien: Option<NaiveDate>,
    pub jenis_kelamin_klien: Option<JenisKelaminEnum>,
    pub agama_klien: Option<String>,
    pub pekerjaan_klien: Option<JenisPekerjaanEnum>,
    pub pendidikan_terakhir_klien: Option<TingkatPendidikanEnum>,
    pub pk_id: i32, // [CHANGE] pk_id WAJIB diisi saat membuat klien. bapas & kanwil akan ikut dari trigger
    pub online_akses_klien: Option<bool>,
    pub pengulangan_klien: Option<bool>,
    pub kewarganegaraan_klien: Option<KewarganegaraanEnum>,
    pub negara_asal_klien: Option<String>,
    pub suku_klien: Option<String>,
    pub keterangan_klien: Option<String>,
    pub catatan_klien: Option<String>,
}

// Data yang dibutuhkan untuk MENGUPDATE Klien (semua opsional)
#[derive(Debug, Deserialize)]
pub struct UpdateKlien {
    pub tipe_klien: Option<TipeKlienEnum>,
    pub nama_klien: Option<String>,
    pub alamat_klien: Option<String>,
    pub tempat_lahir_klien: Option<String>,
    pub tanggal_lahir_klien: Option<NaiveDate>,
    pub jenis_kelamin_klien: Option<JenisKelaminEnum>,
    pub agama_klien: Option<String>,
    pub pekerjaan_klien: Option<JenisPekerjaanEnum>,
    pub pendidikan_terakhir_klien: Option<TingkatPendidikanEnum>,
    pub pk_id: Option<i32>, // Bisa jadi ada pemindahan PK
    pub online_akses_klien: Option<bool>,
    pub pengulangan_klien: Option<bool>,
    pub kewarganegaraan_klien: Option<KewarganegaraanEnum>,
    pub negara_asal_klien: Option<String>,
    pub suku_klien: Option<String>,
    pub keterangan_klien: Option<String>,
    pub catatan_klien: Option<String>,
}
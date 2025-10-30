// src/klien/model_core.rs

// Import the necessary types from our shared types module and chrono
use crate::types::{JenisPekerjaan, KewarganegaraanEnum, TingkatPendidikan, TipeKlien};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// This struct represents a Klien record as it is READ FROM the database.
// It is the complete "source of truth" model.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Klien {
    pub id: i32,
    pub tipe: TipeKlien,
    pub nama: String,
    pub alamat: Option<String>,
    pub tempat_lahir: Option<String>,
    pub tanggal_lahir: Option<NaiveDate>,
    pub jenis_kelamin: Option<String>,
    pub agama: Option<String>,
    pub pekerjaan: Option<JenisPekerjaan>,
    pub pendidikan_terakhir: Option<TingkatPendidikan>,
    pub bapas_id: i32,
    pub pk_id: i32,
    pub online_akses: bool,
    pub pengulangan: bool,
    pub kewarganegaraan: Option<KewarganegaraanEnum>,
    pub negara_asal: Option<String>,
    pub suku: Option<String>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

// This struct represents the JSON payload for CREATING a new Klien.
// Note that `bapas_id` and `pk_id` are optional, as they are determined
// by the backend based on the user's role.
#[derive(Debug, Deserialize)]
pub struct CreateKlien {
    pub tipe: TipeKlien,
    pub nama: String,
    pub alamat: Option<String>,
    pub tempat_lahir: Option<String>,
    pub tanggal_lahir: Option<NaiveDate>,
    pub jenis_kelamin: Option<String>,
    pub agama: Option<String>,
    pub pekerjaan: Option<JenisPekerjaan>,
    pub pendidikan_terakhir: Option<TingkatPendidikan>,
    pub bapas_id: Option<i32>, // Optional for Pegawai/AdminBapas
    pub pk_id: Option<i32>,
    pub kanwil_id: Option<i32>,    // Optional for Pegawai
    pub online_akses: Option<bool>,
    pub pengulangan: Option<bool>,
    pub kewarganegaraan: Option<KewarganegaraanEnum>,
    pub negara_asal: Option<String>,
    pub suku: Option<String>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}

// This struct represents the JSON payload for UPDATING a Klien.
// All fields are optional to allow for partial updates.
#[derive(Debug, Deserialize)]
pub struct UpdateKlien {
    pub tipe: Option<TipeKlien>,
    pub nama: Option<String>,
    pub alamat: Option<String>,
    pub tempat_lahir: Option<String>,
    pub tanggal_lahir: Option<NaiveDate>,
    pub jenis_kelamin: Option<String>,
    pub agama: Option<String>,
    pub pekerjaan: Option<JenisPekerjaan>,
    pub pendidikan_terakhir: Option<TingkatPendidikan>,
    pub bapas_id: Option<i32>,
    pub pk_id: Option<i32>,
    pub online_akses: Option<bool>,
    pub pengulangan: Option<bool>,
    pub kewarganegaraan: Option<KewarganegaraanEnum>,
    pub negara_asal: Option<String>,
    pub suku: Option<String>,
    pub keterangan: Option<String>,
    pub catatan: Option<String>,
}
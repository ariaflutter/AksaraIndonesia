// src/klien/model_core.rs
use crate::types::{JenisPekerjaan, TingkatPendidikan, TipeKlien};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};


// This struct represents a Klien record as it is read FROM the database.
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

// This struct represents the JSON payload sent TO the API to create a new Klien identity.
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
    pub bapas_id: Option<i32>, // <-- Now optional
    pub pk_id: Option<i32>,    // <-- Now optional
}

// --- ADD THIS STRUCT ---
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
}
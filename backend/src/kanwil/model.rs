// File baru: src/kanwil/model.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

// Merepresentasikan satu baris dari tabel 'kanwil'
#[derive(Debug, Serialize, FromRow)]
pub struct Kanwil {
    pub id: i32,
    pub nama_kanwil: String,
    pub alamat_kanwil: Option<String>,
    pub nomor_telepon_kanwil: Option<String>,
    pub email_kanwil: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

// Data yang dibutuhkan untuk membuat Kanwil baru
#[derive(Debug, Deserialize)]
pub struct CreateKanwil {
    pub nama_kanwil: String,
    pub alamat_kanwil: Option<String>,
    pub nomor_telepon_kanwil: Option<String>,
    pub email_kanwil: Option<String>,
}

// Data yang dibutuhkan untuk mengupdate Kanwil (semua opsional)
#[derive(Debug, Deserialize)]
pub struct UpdateKanwil {
    pub nama_kanwil: Option<String>,
    pub alamat_kanwil: Option<String>,
    pub nomor_telepon_kanwil: Option<String>,
    pub email_kanwil: Option<String>,
}
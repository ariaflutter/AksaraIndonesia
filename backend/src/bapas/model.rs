// File baru: src/bapas/model.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

// Merepresentasikan satu baris dari tabel 'bapas'
#[derive(Debug, Serialize, FromRow)]
pub struct Bapas {
    pub id: i32,
    pub kanwil_id: i32,
    pub nama_bapas: String,
    pub kota_bapas: String,
    pub alamat_bapas: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email_bapas: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

// Data yang dibutuhkan untuk membuat Bapas baru
#[derive(Debug, Deserialize)]
pub struct CreateBapas {
    pub kanwil_id: i32,
    pub nama_bapas: String,
    pub kota_bapas: String,
    pub alamat_bapas: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email_bapas: Option<String>,
}

// Data yang dibutuhkan untuk mengupdate Bapas
#[derive(Debug, Deserialize)]
pub struct UpdateBapas {
    pub kanwil_id: Option<i32>,
    pub nama_bapas: Option<String>,
    pub kota_bapas: Option<String>,
    pub alamat_bapas: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email_bapas: Option<String>,
}
// src/bapas/model.rs
use serde::{Deserialize}; // <-- Make sure Deserialize is imported
// This struct defines the data structure for a Bapas office.
// It maps directly to the columns in our 'bapas' database table.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Bapas {
    pub id: i32,
    pub nama_bapas: String,
    pub kota_bapas: String,
    pub alamat_bapas: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email_bapas: Option<String>,
    pub kanwil_id :i32
}

// --- ADD THIS NEW STRUCT ---
// This represents the data the user will send TO our API
// when creating a new Bapas office.
#[derive(Debug, Deserialize)]
pub struct CreateBapas {
    pub nama_bapas: String,
    pub kota_bapas: String,
    pub alamat_bapas: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email_bapas: Option<String>,
    pub kanwil_id :i32,
}
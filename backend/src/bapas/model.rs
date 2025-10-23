// src/bapas/model.rs

// This struct defines the data structure for a Bapas office.
// It maps directly to the columns in our 'bapas' database table.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Bapas {
    pub id: i32,
    pub nama_bapas: String,
    pub kota: String,
    pub alamat: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email: Option<String>,
    pub kanwil :Option<String>,
}
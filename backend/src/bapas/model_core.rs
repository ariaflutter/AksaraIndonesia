// AUTO-GENERATED MODELS FROM DB SCHEMA

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use crate::types::*;

// === Bapas Models ===

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Bapas {
    pub id: i32,
    pub kanwil_id: i32,
    pub nama_bapas: String,
    pub kota_bapas: String,
    pub alamat_bapas: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email_bapas: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateBapas {
    pub kanwil_id: i32,
    pub nama_bapas: String,
    pub kota_bapas: String,
    pub alamat_bapas: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email_bapas: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateBapas {
    pub kanwil_id: Option<i32>,
    pub nama_bapas: Option<String>,
    pub kota_bapas: Option<String>,
    pub alamat_bapas: Option<String>,
    pub nomor_telepon_bapas: Option<String>,
    pub email_bapas: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}


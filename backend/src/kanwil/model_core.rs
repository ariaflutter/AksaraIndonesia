// AUTO-GENERATED MODELS FROM DB SCHEMA

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use crate::types::*;

// === Kanwil Models ===

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Kanwil {
    pub id: i32,
    pub nama_kanwil: String,
    pub alamat_kanwil: Option<String>,
    pub nomor_telepon_kanwil: Option<String>,
    pub email_kanwil: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateKanwil {
    pub nama_kanwil: String,
    pub alamat_kanwil: Option<String>,
    pub nomor_telepon_kanwil: Option<String>,
    pub email_kanwil: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateKanwil {
    pub nama_kanwil: Option<String>,
    pub alamat_kanwil: Option<String>,
    pub nomor_telepon_kanwil: Option<String>,
    pub email_kanwil: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}


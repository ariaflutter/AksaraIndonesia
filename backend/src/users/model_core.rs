// AUTO-GENERATED MODELS FROM DB SCHEMA

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use crate::types::*;

// === Users Models ===

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Users {
    pub id: i32,
    pub nip_user: String,
    pub nama_user: String,
    pub gelar_depan_user: Option<String>,
    pub gelar_belakang_user: Option<String>,
    pub pangkat_golongan_user: Option<String>,
    pub jabatan_user: Option<String>,
    pub bapas_id: Option<i32>,
    pub kanwil_id: Option<i32>,
    pub status_kepegawaian_user: UserStatusKepegawaianEnum,
    pub email_user: Option<String>,
    pub nomor_telepon_user: Option<String>,
    pub status_aktif_user: UserStatusAktifEnum,
    pub role_user: UserRoleEnum,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateUsers {
    pub nip_user: String,
    pub nama_user: String,
    pub gelar_depan_user: Option<String>,
    pub gelar_belakang_user: Option<String>,
    pub pangkat_golongan_user: Option<String>,
    pub jabatan_user: Option<String>,
    pub bapas_id: Option<i32>,
    pub kanwil_id: Option<i32>,
    pub status_kepegawaian_user: UserStatusKepegawaianEnum,
    pub email_user: Option<String>,
    pub nomor_telepon_user: Option<String>,
    pub status_aktif_user: Option<UserStatusAktifEnum>,
    pub role_user: Option<UserRoleEnum>,
    pub password_hash: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUsers {
    pub nip_user: Option<String>,
    pub nama_user: Option<String>,
    pub gelar_depan_user: Option<String>,
    pub gelar_belakang_user: Option<String>,
    pub pangkat_golongan_user: Option<String>,
    pub jabatan_user: Option<String>,
    pub bapas_id: Option<i32>,
    pub kanwil_id: Option<i32>,
    pub status_kepegawaian_user: Option<UserStatusKepegawaianEnum>,
    pub email_user: Option<String>,
    pub nomor_telepon_user: Option<String>,
    pub status_aktif_user: Option<UserStatusAktifEnum>,
    pub role_user: Option<UserRoleEnum>,
    pub password_hash: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}


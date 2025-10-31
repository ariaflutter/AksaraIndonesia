// src/users/model.rs

use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::types::{UserStatusKepegawaianEnum, UserStatusAktifEnum, UserRoleEnum};
use chrono::{DateTime, Utc};

// Struct untuk MENGIRIM data user ke frontend.
// Nama field sekarang 100% sama dengan kolom di database.
#[derive(Debug, Serialize, FromRow)]
pub struct User {
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
    #[serde(skip_serializing)]
    pub api_key_hash: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}


// Struct untuk MENERIMA data dari API untuk membuat user baru.
// Nama field sekarang 100% sama dengan kolom di database.
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub nip_user: String,
    pub nama_user: String,
    pub password: String, // Dibutuhkan untuk membuat hash, bukan kolom DB
    pub gelar_depan_user: Option<String>,
    pub gelar_belakang_user: Option<String>,
    pub pangkat_golongan_user: Option<String>,
    pub jabatan_user: Option<String>,
    pub bapas_id: Option<i32>,
    pub kanwil_id: Option<i32>,
    pub status_kepegawaian_user: UserStatusKepegawaianEnum,
    pub email_user: Option<String>,
    pub nomor_telepon_user: Option<String>,
    pub status_aktif_user: Option<UserStatusAktifEnum>, // Optional karena ada DEFAULT di DB
    pub role_user: UserRoleEnum,
}

// Struct untuk MENERIMA data dari API untuk mengupdate user.
// Nama field sekarang 100% sama dengan kolom di database.
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub nip_user: Option<String>,
    pub nama_user: Option<String>,
    pub password: Option<String>,
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
}

#[derive(Debug, Serialize)]
pub struct ApiKeyStatus {
    pub has_key: bool,
}

// Struct untuk respons POST /api/me/api-key
#[derive(Debug, Serialize)]
pub struct NewApiKey {
    pub api_key: String,
}
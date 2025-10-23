// src/users/model.rs
use serde::Serialize;
use sqlx::FromRow;
use crate::types::{UserStatusKepegawaian, UserStatusAktif, UserRole}; 


// We need to be able to represent the ENUM types from Postgres in Rust.


#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub nip: String,
    pub nama: String,
    pub gelar_depan: Option<String>,
    pub gelar_belakang: Option<String>,
    pub pangkat_golongan: Option<String>,
    pub jabatan: Option<String>,
    pub unit_kerja_id: Option<i32>,
    pub status_kepegawaian: UserStatusKepegawaian,
    pub email: Option<String>,
    pub nomor_telepon: Option<String>,
    pub status_aktif: UserStatusAktif,
    pub role: UserRole,
    // We exclude password_hash from serialization for security.
    #[serde(skip_serializing)]
    pub password_hash: String,
}

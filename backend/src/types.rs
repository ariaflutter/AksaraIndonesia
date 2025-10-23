use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_status_kepegawaian")]
pub enum UserStatusKepegawaian {
    Aktif,
    PindahJabatan,
    Pensiun,
    Lainya,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_status_aktif")]
pub enum UserStatusAktif {
    Aktif,
    Deaktif,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Pegawai,
    AdminBapas,
    SuperAdmin,
}
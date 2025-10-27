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

// --- ADD THESE NEW ENUMS FOR THE 'klien' TABLE ---

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "tipe_klien", rename_all = "PascalCase")]
pub enum TipeKlien {
    Dewasa,
    Anak,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "tingkat_pendidikan")]
pub enum TingkatPendidikan {
    #[serde(rename = "Tidak Sekolah")]
    #[sqlx(rename = "Tidak Sekolah")]
    TidakSekolah,

    #[serde(rename = "SD Tidak Lulus")]
    #[sqlx(rename = "SD Tidak Lulus")]
    SDTidakLulus,

    #[serde(rename = "SD atau Sederajat")]
    #[sqlx(rename = "SD atau Sederajat")]
    SDatauSederajat,
    
    #[serde(rename = "SMP atau Sederajat")]
    #[sqlx(rename = "SMP atau Sederajat")]
    SMPatauSederajat,

    #[serde(rename = "SMA atau Sederajat")]
    #[sqlx(rename = "SMA atau Sederajat")]
    SMAatauSederajat,

    #[serde(rename = "D1 atau Sederajat")]
    #[sqlx(rename = "D1 atau Sederajat")]
    D1atauSederajat,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "jenis_pekerjaan")]
pub enum JenisPekerjaan {
    #[serde(rename = "Belum/Tidak Bekerja")]
    #[sqlx(rename = "Belum/Tidak Bekerja")]
    BelumTidakBekerja,

    #[serde(rename = "Pelajar/Mahasiswa")]
    #[sqlx(rename = "Pelajar/Mahasiswa")]
    PelajarMahasiswa,
    
    PNS,
    
    #[serde(rename = "TNI/Polri")]
    #[sqlx(rename = "TNI/Polri")]
    TNIPolri,

    #[serde(rename = "Karyawan Swasta")]
    #[sqlx(rename = "Karyawan Swasta")]
    KaryawanSwasta,
    
    Wiraswasta,

    #[serde(rename = "Petani/Nelayan")]
    #[sqlx(rename = "Petani/Nelayan")]
    PetaniNelayan,
    
    Lainnya,
}
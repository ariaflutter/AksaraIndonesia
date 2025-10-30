// src/bin/seed_users.rs

use bcrypt::{hash, DEFAULT_COST};
use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Type};
use std::env;
use std::fs::File;
use std::io::BufReader;

// --- Definisi Enum (Tidak Berubah) ---
// Enum ini harus cocok dengan tipe data di PostgreSQL.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_role", rename_all = "PascalCase")]
pub enum UserRole { Pegawai, AdminBapas, SuperAdmin, AdminKanwil }

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_status_kepegawaian", rename_all = "PascalCase")]
pub enum UserStatusKepegawaian { Aktif, PindahJabatan, Pensiun, Lainya }

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_status_aktif", rename_all = "PascalCase")]
pub enum UserStatusAktif { Aktif, Deaktif }

// --- Struct untuk Deserialisasi dari JSON (Tidak Berubah) ---
#[derive(Debug, Deserialize)]
struct UserSeed {
    nama: String,
    gelar_depan: Option<String>,
    gelar_belakang: Option<String>,
    pangkat_golongan: Option<String>,
    jabatan: Option<String>,
    unit_kerja_id: i32,
    status_kepegawaian: UserStatusKepegawaian,
    email: String,
    nomor_telepon: Option<String>,
    status_aktif: UserStatusAktif,
    role: UserRole,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("--- Starting User Seeding from users.json ---");

    // Koneksi ke database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    println!("✓ Database connection established.");

    // Membaca dan mem-parsing file JSON
    let file = File::open("../mockdata/users.json")?;
    let reader = BufReader::new(file);
    let users_from_json: Vec<UserSeed> = serde_json::from_reader(reader)?;
    println!("✓ Found {} users in users.json.", users_from_json.len());

    // Menyiapkan password default yang sudah di-hash
    let password_hash = hash("password123", DEFAULT_COST)?;
    println!("✓ Default password 'password123' hashed.");

    println!("\nInserting users (this may take a moment)...");
    let mut total_inserted = 0;
    
    // Memulai transaksi database untuk efisiensi
    let mut tx = pool.begin().await?;

    for (index, user) in users_from_json.iter().enumerate() {
        // Membuat NIP unik secara dinamis untuk data mock
        let nip = format!("19901010202001{:04}", index + 1);

        // --- QUERY INSERT (TIDAK BERUBAH SECARA LOGIKA) ---
        // Kolom 'kanwil_id' sengaja tidak disertakan.
        // Trigger di database akan mengisinya secara otomatis berdasarkan 'unit_kerja_id'.
        let result = sqlx::query(
            r#"
            INSERT INTO users (
                nip, nama, gelar_depan, gelar_belakang, pangkat_golongan, jabatan,
                unit_kerja_id, status_kepegawaian, email, nomor_telepon,
                status_aktif, role, password_hash
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (nip) DO NOTHING
            "#
        )
        .bind(&nip)
        .bind(&user.nama)
        .bind(&user.gelar_depan)
        .bind(&user.gelar_belakang)
        .bind(&user.pangkat_golongan)
        .bind(&user.jabatan)
        .bind(user.unit_kerja_id) // i32 di-copy, tidak perlu referensi
        .bind(&user.status_kepegawaian)
        .bind(&user.email)
        .bind(&user.nomor_telepon)
        .bind(&user.status_aktif)
        .bind(&user.role)
        .bind(&password_hash)
        .execute(&mut *tx)
        .await?;
        
        total_inserted += result.rows_affected();

        if (index + 1) % 100 == 0 {
            println!("  - Processed {}/{} users...", index + 1, users_from_json.len());
        }
    }
    
    // Menyimpan semua perubahan dalam transaksi ke database
    tx.commit().await?;

    println!("✓ Insertion complete. {} new users were added.", total_inserted);
    println!("\n--- User Seeding Complete! ---");

    Ok(())
}
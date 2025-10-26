// Final corrected version of src/bin/seed_users.rs

use bcrypt::{hash, DEFAULT_COST};
use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Type};
use std::env;
use std::fs::File;
use std::io::BufReader;

// --- Self-Contained Definitions ---
// These enums MUST match the ones in your types.rs,
// including the derive macros, for sqlx::query() to work correctly.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_role", rename_all = "PascalCase")]
pub enum UserRole { Pegawai, AdminBapas, SuperAdmin }

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_status_kepegawaian", rename_all = "PascalCase")]
pub enum UserStatusKepegawaian { Aktif, PindahJabatan, Pensiun, Lainya}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_status_aktif", rename_all = "PascalCase")]
pub enum UserStatusAktif { Aktif, Deaktif }

#[derive(Debug, Deserialize)]
struct UserSeed {
    nama: String,
    gelar_depan: Option<String>,
    gelar_belakang: Option<String>,
    pangkat_golongan: Option<String>,
    jabatan: Option<String>,
    unit_kerja_id: i32,
    status_kepegawaian: UserStatusKepegawaian, // Now reads directly as an enum
    email: String,
    nomor_telepon: Option<String>,
    status_aktif: UserStatusAktif, // Now reads directly as an enum
    role: UserRole, // Now reads directly as an enum
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("--- Starting User Seeding from users.json ---");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    println!("✓ Database connection established.");

    let file = File::open("../mockdata/users.json")?;
    let reader = BufReader::new(file);
    let users_from_json: Vec<UserSeed> = serde_json::from_reader(reader)?;
    println!("✓ Found {} users in users.json.", users_from_json.len());

    let password_hash = hash("password123", DEFAULT_COST)?;
    println!("✓ Default password 'password123' hashed.");

    println!("\nInserting users (this may take a moment)...");
    let mut total_inserted = 0;
    
    let mut tx = pool.begin().await?;

    for (index, user) in users_from_json.iter().enumerate() {
        let nip = format!("19901010202001{:04}", index + 1);

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
        .bind(nip)
        .bind(user.nama.clone())
        .bind(user.gelar_depan.clone())
        .bind(user.gelar_belakang.clone())
        .bind(user.pangkat_golongan.clone())
        .bind(user.jabatan.clone())
        .bind(user.unit_kerja_id.clone())
        .bind(user.status_kepegawaian.clone()) // Now we bind the enum type directly
        .bind(user.email.clone())
        .bind(user.nomor_telepon.clone())
        .bind(user.status_aktif.clone()) // Bind the enum type
        .bind(user.role.clone()) // Bind the enum type
        .bind(&password_hash)
        .execute(&mut *tx)
        .await?;
        
        total_inserted += result.rows_affected();

        if (index + 1) % 100 == 0 {
            println!("  - Processed {}/{} users...", index + 1, users_from_json.len());
        }
    }
    
    tx.commit().await?;

    println!("✓ Insertion complete. {} new users were added.", total_inserted);
    println!("\n--- User Seeding Complete! ---");

    Ok(())
}
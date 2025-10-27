// Final corrected version of src/bin/seed_klien.rs

use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Type};
use std::{env, fs::File, io::BufReader};
use chrono::NaiveDate;

// -----------------------------------------------------------------------------
// ENUM DEFINITIONS — Must match PostgreSQL ENUMs
// -----------------------------------------------------------------------------

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "tipe_klien", rename_all = "PascalCase")]
pub enum TipeKlien {
    Dewasa,
    Anak,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "tingkat_pendidikan", rename_all = "PascalCase")]
pub enum TingkatPendidikan {
    #[serde(rename = "Tidak Sekolah")]
    #[sqlx(rename = "Tidak Sekolah")]
    TidakSekolah,
    #[serde(rename = "SD Tidak Lulus")]
    #[sqlx(rename = "SD Tidak Lulus")]
    SDTidakLulus,
    #[serde(rename = "SD atau Sederajat")]
    #[sqlx(rename = "SD atau Sederajat")]
    SDAtaoSederajat,
    #[serde(rename = "SMP atau Sederajat")]
    #[sqlx(rename = "SMP atau Sederajat")]
    SMPAtauSederajat,
    #[serde(rename = "SMA atau Sederajat")]
    #[sqlx(rename = "SMA atau Sederajat")]
    SMAAtauSederajat,
    #[serde(rename = "D1 atau Sederajat")]
    #[sqlx(rename = "D1 atau Sederajat")]
    D1AtauSederajat,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "jenis_pekerjaan", rename_all = "PascalCase")]
pub enum JenisPekerjaan {
    #[serde(rename = "Belum/Tidak Bekerja")]
    #[sqlx(rename = "Belum/Tidak Bekerja")]
    BelumTidakBekerja,
    #[serde(rename = "Pelajar/Mahasiswa")]
    #[sqlx(rename = "Pelajar/Mahasiswa")]
    PelajarMahasiswa,
    #[serde(rename = "PNS")]
    #[sqlx(rename = "PNS")]
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

// -----------------------------------------------------------------------------
// DATA STRUCT FOR JSON
// -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct KlienSeed {
    tipe: TipeKlien,
    nama: String,
    alamat: Option<String>,
    tempat_lahir: Option<String>,
    tanggal_lahir: Option<NaiveDate>,
    jenis_kelamin: Option<String>,
    agama: Option<String>,
    pekerjaan: Option<JenisPekerjaan>,
    pendidikan_terakhir: Option<TingkatPendidikan>,
    bapas_id: i32,
    pk_id: i32,
}

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("--- Starting Klien Seeding from klien.json ---");

    // 1. Connect to database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    println!("✓ Database connection established.");

    // 2. Read JSON file
    let file = File::open("../mockdata/klien.json")?;
    let reader = BufReader::new(file);
    let klien_list: Vec<KlienSeed> = serde_json::from_reader(reader)?;
    println!("✓ Loaded {} klien records from klien.json.", klien_list.len());

    // 3. Insert data
    println!("\nInserting klien data (this may take a moment)...");
    let mut total_inserted = 0;

    let mut tx = pool.begin().await?;

    for (index, k) in klien_list.iter().enumerate() {
        let result = sqlx::query(
            r#"
            INSERT INTO klien (
                tipe, nama, alamat, tempat_lahir, tanggal_lahir,
                jenis_kelamin, agama, pekerjaan, pendidikan_terakhir,
                bapas_id, pk_id
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(&k.tipe)
        .bind(&k.nama)
        .bind(&k.alamat)
        .bind(&k.tempat_lahir)
        .bind(&k.tanggal_lahir)
        .bind(&k.jenis_kelamin)
        .bind(&k.agama)
        .bind(&k.pekerjaan)
        .bind(&k.pendidikan_terakhir)
        .bind(k.bapas_id)
        .bind(k.pk_id)
        .execute(&mut *tx)
        .await?;

        total_inserted += result.rows_affected();

        if (index + 1) % 100 == 0 {
            println!("  - Processed {}/{} klien...", index + 1, klien_list.len());
        }
    }

    tx.commit().await?;

    println!("✓ Insertion complete. {} new klien added.", total_inserted);
    println!("\n--- Klien Seeding Complete! ---");

    Ok(())
}

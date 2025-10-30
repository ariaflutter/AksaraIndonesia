use chrono::NaiveDate;
use dotenvy::dotenv;
use serde::{Deserialize, Deserializer};
use sqlx::{postgres::PgPoolOptions, Type};
use std::{env, fs::File, io::BufReader};

// -----------------------------------------------------------------------------
// ENUM DEFINITIONS — Must match PostgreSQL ENUMs (but JSON is case-insensitive)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "tipe_klien", rename_all = "PascalCase")]
pub enum TipeKlien {
    Dewasa,
    Anak,
}

// Case-insensitive deserializer for TipeKlien
impl<'de> Deserialize<'de> for TipeKlien {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.trim().to_lowercase().as_str() {
            "dewasa" => Ok(Self::Dewasa),
            "anak" => Ok(Self::Anak),
            _ => Err(serde::de::Error::custom(format!(
                "invalid tipe_klien: '{}'",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "tingkat_pendidikan")]
pub enum TingkatPendidikan {
    #[sqlx(rename = "Tidak Sekolah")]
    TidakSekolah,
    #[sqlx(rename = "SD Tidak Lulus")]
    SdTidakLulus,
    #[sqlx(rename = "SD atau Sederajat")]
    SdAtauSederajat,
    #[sqlx(rename = "SMP atau Sederajat")]
    SmpAtauSederajat,
    #[sqlx(rename = "SMA atau Sederajat")]
    SmaAtauSederajat,
    #[sqlx(rename = "D1 atau Sederajat")]
    D1AtauSederajat,
    #[sqlx(rename = "D2 atau Sederajat")]
    D2AtauSederajat,
    #[sqlx(rename = "D3 atau Sederajat")]
    D3AtauSederajat,
    #[sqlx(rename = "S1 atau Sederajat")]
    S1AtauSederajat,
    #[sqlx(rename = "S2 atau Sederajat")]
    S2AtauSederajat,
    #[sqlx(rename = "S3 atau Sederajat")]
    S3AtauSederajat,
}

// Case-insensitive deserializer for TingkatPendidikan
impl<'de> Deserialize<'de> for TingkatPendidikan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let normalized = s.trim().to_lowercase();
        match normalized.as_str() {
            "tidak sekolah" => Ok(Self::TidakSekolah),
            "sd tidak lulus" => Ok(Self::SdTidakLulus),
            "sd atau sederajat" => Ok(Self::SdAtauSederajat),
            "smp atau sederajat" => Ok(Self::SmpAtauSederajat),
            "sma atau sederajat" => Ok(Self::SmaAtauSederajat),
            "d1 atau sederajat" => Ok(Self::D1AtauSederajat),
            "d2 atau sederajat" => Ok(Self::D2AtauSederajat),
            "d3 atau sederajat" => Ok(Self::D3AtauSederajat),
            "s1 atau sederajat" => Ok(Self::S1AtauSederajat),
            "s2 atau sederajat" => Ok(Self::S2AtauSederajat),
            "s3 atau sederajat" => Ok(Self::S3AtauSederajat),
            _ => Err(serde::de::Error::custom(format!(
                "invalid tingkat_pendidikan: '{}'",
                s
            ))),
        }
    }
}


// REKOMENDASI: Menggunakan ENUM untuk jenis_kelamin
#[derive(Debug, Clone, PartialEq, Eq, Type, Deserialize)]
#[sqlx(type_name = "jenis_kelamin_enum")] // Pastikan ENUM ini ada di DB
pub enum JenisKelamin {
    #[serde(rename = "Laki-laki")]
    LakiLaki,
    Perempuan,
}
// -----------------------------------------------------------------------------
// DATA STRUCT FOR JSON
// -----------------------------------------------------------------------------




// --- Struct untuk JSON (Sudah Disederhanakan) ---
#[derive(Debug, Deserialize)]
struct KlienSeed {
    tipe: TipeKlien,
    nama: String,
    alamat: Option<String>,
    tempat_lahir: Option<String>,
    tanggal_lahir: Option<NaiveDate>, // Menggunakan ENUM
    agama: Option<String>,
    pendidikan_terakhir: Option<TingkatPendidikan>,
    pk_id: i32, // HANYA PK_ID YANG DIBUTUHKAN
    // bapas_id dan kanwil_id dihapus dari sini
}

// --- Fungsi Main (Sudah Disederhanakan) ---
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("--- Starting Klien Seeding from klien.json ---");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    println!("✓ Database connection established.");

    let path = "../mockdata/klien.json";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let klien_list: Vec<KlienSeed> = serde_json::from_reader(reader)?;
    println!("✓ Parsed {} klien from {}.", klien_list.len(), path);

    println!("\nInserting klien data (this may take a moment)...");
    let mut total_inserted = 0;
    let mut tx = pool.begin().await?;

    for (index, k) in klien_list.iter().enumerate() {
        // Query INSERT sekarang lebih sederhana.
        // bapas_id dan kanwil_id akan diisi oleh TRIGGER di database.
        let result = sqlx::query(
            r#"
            INSERT INTO klien (
                tipe, nama, alamat, tempat_lahir, tanggal_lahir,
            agama, pendidikan_terakhir,
                pk_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT DO NOTHING -- Pastikan ada UNIQUE constraint agar ini berfungsi
            "#,
        )
        .bind(&k.tipe)
        .bind(&k.nama)
        .bind(&k.alamat)
        .bind(&k.tempat_lahir)
        .bind(&k.tanggal_lahir)
        .bind(&k.agama)
        .bind(&k.pendidikan_terakhir)
        .bind(k.pk_id) // Hanya bind pk_id
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
// src/bin/seed_kanwil.rs
//
// Run it with:
//    cargo run --bin seed_kanwil
//
// Seeds the `kanwil` table with official data from Dirjen Pemasyarakatan.

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

struct Kanwil {
    nama_kanwil: String,
    alamat_kanwil: Option<String>,
    nomor_telepon_kanwil: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    println!("--- Starting Kanwil Seeding ---");

    // 1. Connect to the Database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    println!("✓ Database connection established.");

    // 2. Define the List of Kanwil
    let kanwil_list = vec![
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Aceh".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Sumatera Utara".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Sumatera Barat".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Riau".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Kepulauan Riau".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Jambi".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Sumatera Selatan".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Kepulauan Bangka Belitung".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Bengkulu".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Lampung".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Daerah Khusus Jakarta".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Jawa Barat".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Banten".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Jawa Tengah".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Daerah Istimewa Yogyakarta".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Jawa Timur".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Kalimantan Barat".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Kalimantan Tengah".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Kalimantan Timur".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Kalimantan Selatan".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Bali".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Nusa Tenggara Barat".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Nusa Tenggara Timur".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Sulawesi Selatan".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Sulawesi Tengah".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Sulawesi Utara".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Gorontalo".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Sulawesi Tenggara".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Maluku".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Maluku Utara".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Papua".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Papua Barat".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
        Kanwil { nama_kanwil: "Kantor Wilayah Direktorat Jenderal Pemasyarakatan Sulawesi Barat".into(), alamat_kanwil: None, nomor_telepon_kanwil: None },
    ];

    println!("✓ Defined {} Kanwil offices to be seeded.", kanwil_list.len());

    // 3. Insert the Data
    println!("\nInserting data into the 'kanwil' table...");
    let mut total_inserted = 0;
    for k in kanwil_list {
        let result = sqlx::query!(
            r#"
            INSERT INTO kanwil (nama_kanwil, alamat_kanwil, nomor_telepon_kanwil)
            VALUES ($1, $2, $3)
            ON CONFLICT (nama_kanwil) DO NOTHING
            "#,
            k.nama_kanwil,
            k.alamat_kanwil,
            k.nomor_telepon_kanwil
        )
        .execute(&pool)
        .await?;

        total_inserted += result.rows_affected();
    }

    println!("✓ Insertion complete. {} new Kanwil records added.", total_inserted);
    println!("\n--- Kanwil Seeding Complete! ---");

    Ok(())
}

// src/bin/seed_bapas.rs

// This is a standalone program to seed the database with a predefined list of Bapas offices.
// Run it with: `cargo run --bin seed_bapas`

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

// Define a simple struct to hold our Bapas data.
// We don't need the full model from the main app, just what's needed for insertion.
struct Bapas {
    nama_bapas: String,
    kota: String,
    alamat: Option<String>,
    nomor_telepon_bapas: Option<String>,
    email: Option<String>,
    kanwil: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    println!("--- Starting Bapas Seeding ---");

    // --- 1. Connect to the Database ---
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    println!("✓ Database connection established.");

    // --- 2. Define the List of Bapas ---
    // This is the real data you provided.
    let bapas_list = vec![
    Bapas { nama_bapas: "Bapas Jakarta Pusat".to_string(), kota: "Jakarta Pusat".to_string(), alamat: Some("Jl. Gunung Sahari No. 17, Jakarta Pusat".to_string()), nomor_telepon_bapas: Some("(021) 3921020".to_string()), email: Some("bapas.jakpus@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil DKI Jakarta".to_string()) },
    Bapas { nama_bapas: "Bapas Jakarta Timur".to_string(), kota: "Jakarta Timur".to_string(), alamat: Some("Jl. Raya Bekasi KM 18, Jakarta Timur".to_string()), nomor_telepon_bapas: Some("(021) 4602911".to_string()), email: Some("bapas.jaktim@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil DKI Jakarta".to_string()) },
    Bapas { nama_bapas: "Bapas Jakarta Barat".to_string(), kota: "Jakarta Barat".to_string(), alamat: Some("Jl. S. Parman No. 21, Jakarta Barat".to_string()), nomor_telepon_bapas: Some("(021) 5672341".to_string()), email: Some("bapas.jakbar@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil DKI Jakarta".to_string()) },
    Bapas { nama_bapas: "Bapas Jakarta Selatan".to_string(), kota: "Jakarta Selatan".to_string(), alamat: Some("Jl. Warung Buncit Raya No. 22".to_string()), nomor_telepon_bapas: Some("(021) 7980123".to_string()), email: Some("bapas.jaksel@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil DKI Jakarta".to_string()) },
    Bapas { nama_bapas: "Bapas Jakarta Utara".to_string(), kota: "Jakarta Utara".to_string(), alamat: Some("Jl. Yos Sudarso No. 45".to_string()), nomor_telepon_bapas: Some("(021) 6519871".to_string()), email: Some("bapas.jakut@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil DKI Jakarta".to_string()) },

    Bapas { nama_bapas: "Bapas Bandung".to_string(), kota: "Bandung".to_string(), alamat: Some("Jl. Soekarno-Hatta No. 101, Bandung".to_string()), nomor_telepon_bapas: Some("(022) 7561234".to_string()), email: Some("bapas.bandung@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Barat".to_string()) },
    Bapas { nama_bapas: "Bapas Bogor".to_string(), kota: "Bogor".to_string(), alamat: Some("Jl. Pajajaran No. 87, Bogor".to_string()), nomor_telepon_bapas: Some("(0251) 8342345".to_string()), email: Some("bapas.bogor@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Barat".to_string()) },
    Bapas { nama_bapas: "Bapas Cirebon".to_string(), kota: "Cirebon".to_string(), alamat: Some("Jl. Tuparev No. 23, Cirebon".to_string()), nomor_telepon_bapas: Some("(0231) 208710".to_string()), email: Some("bapas.cirebon@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Barat".to_string()) },
    Bapas { nama_bapas: "Bapas Sukabumi".to_string(), kota: "Sukabumi".to_string(), alamat: Some("Jl. Ahmad Yani No. 10, Sukabumi".to_string()), nomor_telepon_bapas: None, email: Some("bapas.sukabumi@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Barat".to_string()) },
    Bapas { nama_bapas: "Bapas Tasikmalaya".to_string(), kota: "Tasikmalaya".to_string(), alamat: Some("Jl. HZ Mustofa No. 12, Tasikmalaya".to_string()), nomor_telepon_bapas: None, email: Some("bapas.tasik@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Barat".to_string()) },

    Bapas { nama_bapas: "Bapas Semarang".to_string(), kota: "Semarang".to_string(), alamat: Some("Jl. Dr. Cipto No. 11, Semarang".to_string()), nomor_telepon_bapas: Some("(024) 3543342".to_string()), email: Some("bapas.semarang@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Tengah".to_string()) },
    Bapas { nama_bapas: "Bapas Surakarta".to_string(), kota: "Surakarta".to_string(), alamat: Some("Jl. Slamet Riyadi No. 221, Solo".to_string()), nomor_telepon_bapas: None, email: Some("bapas.surakarta@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Tengah".to_string()) },
    Bapas { nama_bapas: "Bapas Pekalongan".to_string(), kota: "Pekalongan".to_string(), alamat: Some("Jl. Dr. Sutomo No. 17, Pekalongan".to_string()), nomor_telepon_bapas: None, email: Some("bapas.pekalongan@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Tengah".to_string()) },
    Bapas { nama_bapas: "Bapas Pati".to_string(), kota: "Pati".to_string(), alamat: Some("Jl. Pati-Kudus Km 3".to_string()), nomor_telepon_bapas: None, email: Some("bapas.pati@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Tengah".to_string()) },
    Bapas { nama_bapas: "Bapas Purwokerto".to_string(), kota: "Purwokerto".to_string(), alamat: Some("Jl. Gatot Subroto No. 12, Purwokerto".to_string()), nomor_telepon_bapas: None, email: Some("bapas.purwokerto@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Tengah".to_string()) },

    Bapas { nama_bapas: "Bapas Yogyakarta".to_string(), kota: "Yogyakarta".to_string(), alamat: Some("Jl. Kusumanegara No. 7, Yogyakarta".to_string()), nomor_telepon_bapas: Some("(0274) 515514".to_string()), email: Some("bapas.yogyakarta@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil DIY".to_string()) },
    Bapas { nama_bapas: "Bapas Surabaya".to_string(), kota: "Surabaya".to_string(), alamat: Some("Jl. Raya Medokan No. 33, Surabaya".to_string()), nomor_telepon_bapas: Some("(031) 8490221".to_string()), email: Some("bapas.surabaya@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Timur".to_string()) },
    Bapas { nama_bapas: "Bapas Malang".to_string(), kota: "Malang".to_string(), alamat: Some("Jl. Veteran No. 18, Malang".to_string()), nomor_telepon_bapas: Some("(0341) 324556".to_string()), email: Some("bapas.malang@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Timur".to_string()) },
    Bapas { nama_bapas: "Bapas Kediri".to_string(), kota: "Kediri".to_string(), alamat: Some("Jl. Pahlawan No. 8, Kediri".to_string()), nomor_telepon_bapas: None, email: Some("bapas.kediri@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Timur".to_string()) },
    Bapas { nama_bapas: "Bapas Madiun".to_string(), kota: "Madiun".to_string(), alamat: Some("Jl. S. Parman No. 11, Madiun".to_string()), nomor_telepon_bapas: None, email: Some("bapas.madiun@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Timur".to_string()) },
    Bapas { nama_bapas: "Bapas Banyuwangi".to_string(), kota: "Banyuwangi".to_string(), alamat: Some("Jl. Jaksa Agung Suprapto No. 45".to_string()), nomor_telepon_bapas: None, email: Some("bapas.banyuwangi@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Jawa Timur".to_string()) },

    Bapas { nama_bapas: "Bapas Denpasar".to_string(), kota: "Denpasar".to_string(), alamat: Some("Jl. Gunung Agung No. 21, Denpasar".to_string()), nomor_telepon_bapas: Some("(0361) 427222".to_string()), email: Some("bapas.denpasar@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Bali".to_string()) },
    Bapas { nama_bapas: "Bapas Mataram".to_string(), kota: "Mataram".to_string(), alamat: Some("Jl. Langko No. 19, Mataram".to_string()), nomor_telepon_bapas: None, email: Some("bapas.mataram@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil NTB".to_string()) },
    Bapas { nama_bapas: "Bapas Kupang".to_string(), kota: "Kupang".to_string(), alamat: Some("Jl. El Tari No. 55, Kupang".to_string()), nomor_telepon_bapas: None, email: Some("bapas.kupang@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil NTT".to_string()) },
    Bapas { nama_bapas: "Bapas Pontianak".to_string(), kota: "Pontianak".to_string(), alamat: Some("Jl. Sisingamangaraja No. 11, Pontianak".to_string()), nomor_telepon_bapas: None, email: Some("bapas.pontianak@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Kalimantan Barat".to_string()) },
    Bapas { nama_bapas: "Bapas Banjarmasin".to_string(), kota: "Banjarmasin".to_string(), alamat: Some("Jl. A. Yani Km 5, Banjarmasin".to_string()), nomor_telepon_bapas: None, email: Some("bapas.banjarmasin@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Kalimantan Selatan".to_string()) },
    Bapas { nama_bapas: "Bapas Palangkaraya".to_string(), kota: "Palangkaraya".to_string(), alamat: Some("Jl. Diponegoro No. 18".to_string()), nomor_telepon_bapas: None, email: Some("bapas.palangkaraya@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Kalimantan Tengah".to_string()) },
    Bapas { nama_bapas: "Bapas Samarinda".to_string(), kota: "Samarinda".to_string(), alamat: Some("Jl. Juanda No. 15, Samarinda".to_string()), nomor_telepon_bapas: None, email: Some("bapas.samarinda@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Kalimantan Timur".to_string()) },
    Bapas { nama_bapas: "Bapas Balikpapan".to_string(), kota: "Balikpapan".to_string(), alamat: Some("Jl. MT Haryono No. 90".to_string()), nomor_telepon_bapas: None, email: Some("bapas.balikpapan@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Kalimantan Timur".to_string()) },
    Bapas { nama_bapas: "Bapas Tarakan".to_string(), kota: "Tarakan".to_string(), alamat: Some("Jl. Mulawarman No. 12, Tarakan".to_string()), nomor_telepon_bapas: None, email: Some("bapas.tarakan@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Kalimantan Utara".to_string()) },

    Bapas { nama_bapas: "Bapas Makassar".to_string(), kota: "Makassar".to_string(), alamat: Some("Jl. Perintis Kemerdekaan No. 7".to_string()), nomor_telepon_bapas: None, email: Some("bapas.makassar@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Sulawesi Selatan".to_string()) },
    Bapas { nama_bapas: "Bapas Parepare".to_string(), kota: "Parepare".to_string(), alamat: Some("Jl. Jend. Sudirman No. 18".to_string()), nomor_telepon_bapas: None, email: Some("bapas.parepare@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Sulawesi Selatan".to_string()) },
    Bapas { nama_bapas: "Bapas Palu".to_string(), kota: "Palu".to_string(), alamat: Some("Jl. Sam Ratulangi No. 55".to_string()), nomor_telepon_bapas: None, email: Some("bapas.palu@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Sulawesi Tengah".to_string()) },
    Bapas { nama_bapas: "Bapas Gorontalo".to_string(), kota: "Gorontalo".to_string(), alamat: Some("Jl. Ahmad Yani No. 19".to_string()), nomor_telepon_bapas: None, email: Some("bapas.gorontalo@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Gorontalo".to_string()) },
    Bapas { nama_bapas: "Bapas Manado".to_string(), kota: "Manado".to_string(), alamat: Some("Jl. Piere Tendean No. 88".to_string()), nomor_telepon_bapas: None, email: Some("bapas.manado@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Sulawesi Utara".to_string()) },
    Bapas { nama_bapas: "Bapas Kendari".to_string(), kota: "Kendari".to_string(), alamat: Some("Jl. Lawata No. 11".to_string()), nomor_telepon_bapas: None, email: Some("bapas.kendari@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Sulawesi Tenggara".to_string()) },
    Bapas { nama_bapas: "Bapas Ambon".to_string(), kota: "Ambon".to_string(), alamat: Some("Jl. Sultan Babullah No. 9".to_string()), nomor_telepon_bapas: None, email: Some("bapas.ambon@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Maluku".to_string()) },
    Bapas { nama_bapas: "Bapas Ternate".to_string(), kota: "Ternate".to_string(), alamat: Some("Jl. Ahmad Yani No. 21".to_string()), nomor_telepon_bapas: None, email: Some("bapas.ternate@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Maluku Utara".to_string()) },
    Bapas { nama_bapas: "Bapas Jayapura".to_string(), kota: "Jayapura".to_string(), alamat: Some("Jl. Koti No. 7".to_string()), nomor_telepon_bapas: None, email: Some("bapas.jayapura@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Papua".to_string()) },
    Bapas { nama_bapas: "Bapas Merauke".to_string(), kota: "Merauke".to_string(), alamat: Some("Jl. Mandala No. 9".to_string()), nomor_telepon_bapas: None, email: Some("bapas.merauke@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Papua Selatan".to_string()) },
    Bapas { nama_bapas: "Bapas Manokwari".to_string(), kota: "Manokwari".to_string(), alamat: Some("Jl. Trikora No. 12".to_string()), nomor_telepon_bapas: None, email: Some("bapas.manokwari@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Papua Barat".to_string()) },
    Bapas { nama_bapas: "Bapas Sorong".to_string(), kota: "Sorong".to_string(), alamat: Some("Jl. Basuki Rahmat No. 10".to_string()), nomor_telepon_bapas: None, email: Some("bapas.sorong@kemenkumham.go.id".to_string()), kanwil: Some("Kanwil Papua Barat Daya".to_string()) },
    ];
    println!("✓ Defined {} Bapas offices to be seeded.", bapas_list.len());

    // --- 3. Insert the Data into the Database ---
    println!("\nInserting data into the 'bapas' table...");
    let mut total_inserted = 0;
    for bapas in bapas_list {
        // The query will attempt to insert a new bapas.
        // `ON CONFLICT (nama_bapas) DO NOTHING` is a powerful PostgreSQL feature.
        // It means: if a bapas with this name already exists, just skip it and don't raise an error.
        // This makes the script safe to run multiple times.
        let result = sqlx::query!(
            r#"
            INSERT INTO bapas (nama_bapas, kota, alamat, nomor_telepon_bapas, email, kanwil)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (nama_bapas) DO NOTHING
            "#,
            bapas.nama_bapas,
            bapas.kota,
            bapas.alamat,
            bapas.nomor_telepon_bapas,
            bapas.email,
            bapas.kanwil
        )
        .execute(&pool)
        .await?;

        total_inserted += result.rows_affected();
    }

    println!("✓ Insertion complete. {} new Bapas offices were added.", total_inserted);
    println!("\n--- Bapas Seeding Complete! ---");

    Ok(())
}
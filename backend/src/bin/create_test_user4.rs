use bcrypt::{hash, DEFAULT_COST};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 👇 Customize these values as needed
    let nip_user = "ariaflutter4";
    let password = "password123";
    let nama_user = "Gema Pegawai";

    // Example IDs — make sure these exist in your DB
    let kanwil_id: Option<i32> = Some(1);
    let bapas_id: Option<i32> = Some(1);

    let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");

    println!("🚀 Creating Pegawai user...");
    println!("NIP: {}", nip_user);
    println!("Password: {}", password);

    let result = sqlx::query!(
        r#"
        INSERT INTO users (
            nip_user, 
            nama_user, 
            password_hash, 
            role_user, 
            status_kepegawaian_user, 
            status_aktif_user,
            kanwil_id,
            bapas_id
        )
        VALUES (
            $1, 
            $2, 
            $3, 
            'Pegawai'::user_role_enum, 
            'Aktif'::user_status_kepegawaian_enum, 
            'Aktif'::user_status_aktif_enum,
            $4,
            $5
        )
        "#,
        nip_user,
        nama_user,
        password_hash,
        kanwil_id,
        bapas_id
    )
    .execute(&pool)
    .await?;

    println!("✅ User created successfully! {} rows affected.", result.rows_affected());
    Ok(())
}

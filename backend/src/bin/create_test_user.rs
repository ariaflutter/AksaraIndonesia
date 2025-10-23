// src/bin/create_test_user.rs
use bcrypt::{hash, DEFAULT_COST};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    let nip = "ariaflutter";
    let password = "password123";
    let nama = "Super Admin";
    
    let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");

    println!("Creating SuperAdmin user...");
    println!("NIP: {}", nip);
    println!("Password: {}", password);

    // CORRECTED QUERY with the new enum values
    let result = sqlx::query!(
        r#"
        INSERT INTO users (nip, nama, password_hash, role, status_kepegawaian, status_aktif)
        VALUES ($1, $2, $3, 'SuperAdmin'::user_role, 'Aktif'::user_status_kepegawaian, 'Aktif'::user_status_aktif)
        "#,
        nip,
        nama,
        password_hash
    )
    .execute(&pool)
    .await?;

    println!("User created successfully! {} rows affected.", result.rows_affected());
    Ok(())
}
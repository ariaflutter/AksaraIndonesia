// src/main.rs
// Declare the modules we created. Rust will look for /bapas/mod.rs and /routes/mod.rs.
mod types;
mod bapas;
mod routes;
mod users;
mod auth; 

use axum::{extract::Extension, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tracing::info;
#[tokio::main]
async fn main() {
dotenv().ok();
tracing_subscriber::fmt().with_target(false).with_env_filter("info").init();
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect to the database");

info!("Database connection pool established successfully.");

// The main application router.
let app = Router::new()
    // All API routes will be nested under "/api".
    .nest("/api", routes::create_api_router())
    // Share the database pool with all routes.
    .layer(Extension(pool));

let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
info!("Aksara Backend listening on http://{}", addr);

let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
axum::serve(listener, app).await.unwrap();
}
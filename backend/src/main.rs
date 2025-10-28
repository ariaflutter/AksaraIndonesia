// src/main.rs
// Declare the modules we created. Rust will look for /bapas/mod.rs and /routes/mod.rs.
pub mod types;
mod bapas;
mod routes;
mod users;
mod auth; 
mod klien;
pub mod utils;

use axum::{extract::Extension, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use std::time::Duration; 
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;
use tracing::info;
#[tokio::main]
async fn main() {
dotenv().ok();
tracing_subscriber::fmt().with_target(false).with_env_filter("info").init();
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(3)) // Timeout for getting a connection from the pool
    .idle_timeout(Duration::from_secs(30)) // Close idle connections after 30s
    .test_before_acquire(true) // Ping the DB before handing out a connection
    .connect(&database_url)
    .await
    .expect("Failed to connect to the database");

info!("Database connection pool established successfully.");



// Set up CORS
    // --- NEW: CREATE THE CORS LAYER ---
    // This defines the permissions for cross-origin requests.
    let cors = CorsLayer::new()
        // Allow requests from our Svelte frontend's origin.
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        // Allow common HTTP methods.
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // Allow the browser to send the 'Content-Type' and 'Authorization' headers.
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION]);

    // The main application router.
    let app = Router::new()
        // All API routes will be nested under "/api".
        .nest("/api", routes::create_api_router())
        // Share the database pool with all routes.
        .layer(Extension(pool))
        // --- APPLY THE CORS LAYER TO THE ENTIRE APP ---
        .layer(cors);

let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
info!("Aksara Backend listening on http://{}", addr);

let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
axum::serve(listener, app).await.unwrap();
}
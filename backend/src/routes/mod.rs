// in src/routes/mod.rs
use axum::{middleware, routing::{get, post}, Router};
use crate::{bapas, users, auth};

pub fn create_api_router() -> Router {
    // These routes are PROTECTED and require a valid JWT.
    // We apply our `auth` middleware function to this router.
    let protected_router = Router::new()
        .route("/users", get(users::handlers::get_all_users))
        // Add other protected routes here in the future
        .layer(middleware::from_fn(auth::middleware::auth));

    // This is the main router.
    Router::new()
        // Public routes (like login) do NOT go inside the protected router.
        .route("/auth/login", post(auth::handlers::login))
        // Publicly accessible bapas list
        .route("/bapas", get(bapas::handlers::get_all_bapas))
        // Nest all the protected routes under the main router.
        .nest("/", protected_router)
}
// in src/routes/mod.rs
use axum::{middleware, routing::{get, post}, Router};
use crate::{bapas, users, auth};

pub fn create_api_router() -> Router {
    // These routes are PROTECTED and require a valid JWT.
    // We apply our `auth` middleware function to this router.
    let protected_router = Router::new()
        .route("/bapas", get(bapas::handlers::get_all_bapas).post(bapas::handlers::create_bapas))
        .route("/bapas/:id", get(bapas::handlers::get_bapas_by_id).put(bapas::handlers::update_bapas).delete(bapas::handlers::delete_bapas))
         // Users Routes
        .route("/users", get(users::handlers::get_all_users).post(users::handlers::create_user))
        .route("/users/:id",
            get(users::handlers::get_user_by_id).delete(users::handlers::delete_user).put(users::handlers::update_user),
            
        )
        // Add other protected routes here in the future
        .layer(middleware::from_fn(auth::middleware::auth));

    // This is the main router.
    Router::new()
        // Public routes (like login) do NOT go inside the protected router.
        .route("/auth/login", post(auth::handlers::login))
        // Publicly accessible bapas list
        
        // Nest all the protected routes under the main router.
        .nest("/", protected_router)
}
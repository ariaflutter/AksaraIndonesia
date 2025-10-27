// in src/routes/mod.rs
use axum::{middleware, routing::{get, post}, Router};
use crate::{bapas, users, auth, klien};

pub fn create_api_router() -> Router {
    // These routes are PROTECTED and require a valid JWT.
    // We apply our `auth` middleware function to this router.
    let protected_router = Router::new()
        .route("/auth/me", get(auth::handlers::me))
        .route("/bapas", get(bapas::handlers::get_all_bapas).post(bapas::handlers::create_bapas))
        .route("/bapas/:id", get(bapas::handlers::get_bapas_by_id).put(bapas::handlers::update_bapas).delete(bapas::handlers::delete_bapas))
         // Users Routes
        .route("/users", get(users::handlers::get_all_users).post(users::handlers::create_user))
        .route("/users/:id",
            get(users::handlers::get_user_by_id).delete(users::handlers::delete_user).put(users::handlers::update_user),)
        .route("/klien", 
            get(klien::handlers_core::get_all_klien)
            .post(klien::handlers_core::create_klien)
        )
        .route("/klien/:id",
            get(klien::handlers_core::get_klien_by_id)
                .put(klien::handlers_core::update_klien)
                .delete(klien::handlers_core::delete_klien)
        )
         .route(
            "/klien/:klien_id/penerimaan-dewasa",
            post(klien::handlers_dewasa::create_penerimaan_dewasa)
                .get(klien::handlers_dewasa::get_all_penerimaan_for_klien),
        )
        // Routes for a specific item, accessed directly by its own ID
        .route(
            "/penerimaan-dewasa/:id",
            // We are missing a `get_penerimaan_by_id` handler, let's add it
            get(klien::handlers_dewasa::get_penerimaan_by_id)
            .put(klien::handlers_dewasa::update_penerimaan_dewasa)
                .delete(klien::handlers_dewasa::delete_penerimaan_dewasa),
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
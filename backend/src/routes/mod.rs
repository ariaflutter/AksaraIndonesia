// in src/routes/mod.rs
use axum::{middleware, routing::{get, post, put, delete}, Router};
use crate::{ users, auth, bapas, kanwil, klien};
use crate::auth::middleware::{
    self as auth_middleware, // Gunakan alias untuk middleware utama
    authorize_klien_access,
    authorize_petugas_lapor_access,
    authorize_wajib_lapor_delete_access,
};
use axum::extract::Path;

//use crate::{bapas, users, auth, klien};
pub fn create_api_router() -> Router {
    // These routes are PROTECTED and require a valid JWT.
    // We apply our `auth` middleware function to this router.
    let protected_router = Router::new()
        .route("/auth/me", get(auth::handlers::me))


         // --- KANWIL Routes (Hanya untuk SuperAdmin) ---
        .route("/kanwil", get(kanwil::handlers::get_all_kanwil).post(kanwil::handlers::create_kanwil))
        .route("/kanwil/:id", get(kanwil::handlers::get_kanwil_by_id).put(kanwil::handlers::update_kanwil).delete(kanwil::handlers::delete_kanwil))
        
        
        // --- BAPAS Routes (Hanya untuk SuperAdmin dan Kanwil Admin) ---
        .route("/bapas", get(bapas::handlers::get_all_bapas).post(bapas::handlers::create_bapas))
        .route("/bapas/:id", get(bapas::handlers::get_bapas_by_id).put(bapas::handlers::update_bapas).delete(bapas::handlers::delete_bapas))


         // Users Routes
        .route("/users", get(users::handlers::get_all_users).post(users::handlers::create_user))
        .route("/users/:id",
            get(users::handlers::get_user_by_id).delete(users::handlers::delete_user).put(users::handlers::update_user),)


        // --- KLIEN CORE ---
        .route("/klien", get(klien::handlers_core::get_all_klien).post(klien::handlers_core::create_klien))
        .route("/klien/:id", get(klien::handlers_core::get_klien_by_id).put(klien::handlers_core::update_klien).delete(klien::handlers_core::delete_klien))

        // --- PENERIMAAN DEWASA ---
        .route(
            "/klien/:klien_id/penerimaan-dewasa",
            get(klien::handlers_dewasa::get_all_penerimaan_for_klien)
                .post(klien::handlers_dewasa::create_penerimaan_dewasa),
        )
        .route(
            "/penerimaan-dewasa/:id",
            get(klien::handlers_dewasa::get_penerimaan_by_id)
                .put(klien::handlers_dewasa::update_penerimaan_dewasa)
                .delete(klien::handlers_dewasa::delete_penerimaan_dewasa),
        )


         // --- RIWAYAT HUKUM DEWASA (BARU) ---
        .route(
            "/klien/:klien_id/riwayat-hukum-dewasa",
            get(klien::handlers_dewasa::get_all_riwayat_hukum_for_klien)
                .post(klien::handlers_dewasa::create_riwayat_hukum_dewasa),
        )
        .route(
            "/riwayat-hukum-dewasa/:id",
            get(klien::handlers_dewasa::get_riwayat_hukum_by_id)
                .put(klien::handlers_dewasa::update_riwayat_hukum_dewasa)
                .delete(klien::handlers_dewasa::delete_riwayat_hukum_dewasa),
        )





 // --- LAYANAN INTEGRASI DEWASA (BARU) ---
        .route(
            "/klien/:klien_id/layanan-integrasi-dewasa",
            get(klien::handlers_dewasa::get_all_layanan_integrasi_for_klien)
                .post(klien::handlers_dewasa::create_layanan_integrasi_dewasa),
        )
        .route(
            "/layanan-integrasi-dewasa/:id",
            get(klien::handlers_dewasa::get_layanan_integrasi_by_id)
                .put(klien::handlers_dewasa::update_layanan_integrasi_dewasa)
                .delete(klien::handlers_dewasa::delete_layanan_integrasi_dewasa),
        )






          // --- PROSES HUKUM DEWASA (BARU) ---
        .route(
            "/penerimaan-dewasa/:penerimaan_id/proses-hukum-dewasa",
            get(klien::handlers_dewasa::get_all_proses_hukum_for_penerimaan)
                .post(klien::handlers_dewasa::create_proses_hukum_dewasa),
        )
        .route(
            "/proses-hukum-dewasa/:id",
            get(klien::handlers_dewasa::get_proses_hukum_by_id)
                .put(klien::handlers_dewasa::update_proses_hukum_dewasa)
                .delete(klien::handlers_dewasa::delete_proses_hukum_dewasa),
        )



         // --- WAJIB LAPOR (PROTECTED) ---
        .route("/klien/:klien_id/wajib-lapor-dewasa", get(klien::handlers_dewasa::get_all_wajib_lapor_for_klien))
        .route_layer(middleware::from_fn(authorize_klien_access)) // Lindungi get all
        
        .route("/petugas/klien/:klien_id/wajib-lapor-dewasa", post(klien::handlers_dewasa::petugas_wajib_lapor_dewasa))
        .route_layer(middleware::from_fn(authorize_petugas_lapor_access)) // Lindungi lapor petugas

        .route("/kiosk/klien/:klien_id/wajib-lapor-dewasa", post(klien::handlers_dewasa::kiosk_wajib_lapor_dewasa))
        .route_layer(middleware::from_fn(authorize_klien_access)) // Kiosk pakai aturan standar

        .route("/wajib-lapor-dewasa/:id", delete(klien::handlers_dewasa::delete_wajib_lapor_dewasa))
        .route_layer(middleware::from_fn(authorize_wajib_lapor_delete_access)) // Lindungi delete


        .layer(middleware::from_fn(auth_middleware::auth)); // Gunakan alias

    // This is the main router.
    Router::new()
        // Public routes (like login) do NOT go inside the protected router.
        .route("/auth/login", post(auth::handlers::login))
        .route("/mandiri/klien/:klien_id/wajib-lapor-dewasa", post(klien::handlers_dewasa::mandiri_wajib_lapor_dewasa))
        .nest("/", protected_router)
}
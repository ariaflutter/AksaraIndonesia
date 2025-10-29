// src/klien/handlers_core.rs

use crate::auth::model::Claims;
use crate::klien::model_core::{CreateKlien, Klien, UpdateKlien};
use crate::utils::Pagination;
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Postgres};

// --- Authorization Helper ---
// This struct and its methods contain all our security rules for accessing a Klien.
struct AuthContext<'a> {
    pool: &'a PgPool,
    klien: &'a Klien,
    claims: &'a Claims,
}

impl<'a> AuthContext<'a> {
    fn new(pool: &'a PgPool, klien: &'a Klien, claims: &'a Claims) -> Self {
        Self { pool, klien, claims }
    }

    // Inside the `is_authorized` method
async fn is_authorized(&self) -> Result<bool, StatusCode> {
    match self.claims.role {
        crate::types::UserRole::SuperAdmin => Ok(true),
        crate::types::UserRole::AdminKanwil => {
            let admin_kanwil_id = match self.claims.kanwil_id {
                Some(id) => id,
                None => return Ok(false), // AdminKanwil with no Kanwil cannot see anything.
            };

            // FIX 1: Combine the query and the final return into one logical expression
            let bapas_kanwil_id: Option<i32> = sqlx::query_scalar!(
                "SELECT kanwil_id FROM bapas WHERE id = $1",
                self.klien.bapas_id
            )
            .fetch_optional(self.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(bapas_kanwil_id == Some(admin_kanwil_id))
        }
        crate::types::UserRole::AdminBapas => {
            // A slightly safer way to write this
            if let Some(user_bapas_id) = self.claims.unit_kerja_id {
                Ok(self.klien.bapas_id == user_bapas_id)
            } else {
                Ok(false)
            }
        }
        crate::types::UserRole::Pegawai => Ok(self.klien.pk_id == self.claims.sub),
    }
}
}

pub async fn create_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateKlien>,
) -> Result<Json<Klien>, StatusCode> {
    
    let created_by_id = claims.sub;

    // --- REVISED: AUTHORIZATION AND DATA SCOPING LOGIC ---
    let target_bapas_id: i32;
    let target_pk_id: i32;
    let target_kanwil_id: i32;

    match claims.role {
        crate::types::UserRole::SuperAdmin => {
            target_bapas_id = payload.bapas_id.ok_or(StatusCode::BAD_REQUEST)?;
            target_pk_id = payload.pk_id.ok_or(StatusCode::BAD_REQUEST)?;
        }
        // FIX 2: Add the correct logic for AdminKanwil
        crate::types::UserRole::AdminKanwil => {
            // AdminKanwil must provide bapas_id and pk_id.
            let bapas_id_from_payload = payload.bapas_id.ok_or(StatusCode::BAD_REQUEST)?;
            target_pk_id = payload.pk_id.ok_or(StatusCode::BAD_REQUEST)?;

            // SECURITY CHECK: Verify the provided bapas_id belongs to the admin's kanwil.
            let admin_kanwil_id = claims.kanwil_id.ok_or(StatusCode::FORBIDDEN)?;
            let bapas_kanwil_id: Option<i32> = sqlx::query_scalar!(
                "SELECT kanwil_id FROM bapas WHERE id = $1",
                bapas_id_from_payload
            )
            .fetch_optional(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            if bapas_kanwil_id == Some(admin_kanwil_id) {
                // The check passed. We can use the bapas_id from the payload.
                target_bapas_id = bapas_id_from_payload;
            } else {
                // The provided Bapas is not in the admin's Kanwil.
                return Err(StatusCode::FORBIDDEN);
            }
        }
        crate::types::UserRole::AdminBapas => {
            target_bapas_id = claims.unit_kerja_id.ok_or(StatusCode::FORBIDDEN)?;
            target_pk_id = payload.pk_id.ok_or(StatusCode::BAD_REQUEST)?;
        }
        crate::types::UserRole::Pegawai => {
            target_bapas_id = claims.unit_kerja_id.ok_or(StatusCode::FORBIDDEN)?;
            target_pk_id = claims.sub;
        }
    }
    // ---------------------------------------------------

    // Now, we proceed with the insertion, but we use our verified `target_bapas_id`.
    let result = sqlx::query_as(
        r#"
        INSERT INTO klien (
            tipe, nama, alamat, tempat_lahir, tanggal_lahir, jenis_kelamin, agama,
            pekerjaan, pendidikan_terakhir, bapas_id, pk_id, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
    )
    .bind(&payload.tipe)
    .bind(&payload.nama)
    .bind(&payload.alamat)
    .bind(&payload.tempat_lahir)
    .bind(payload.tanggal_lahir)
    .bind(&payload.jenis_kelamin)
    .bind(&payload.agama)
    .bind(&payload.pekerjaan)
    .bind(payload.pendidikan_terakhir)
    .bind(target_bapas_id) // <-- USE THE VERIFIED ID, NOT THE PAYLOAD ID
    .bind(target_pk_id)
    .bind(created_by_id)
    .bind(created_by_id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(new_klien) => Ok(Json(new_klien)),
        Err(e) => {
            tracing::error!("Failed to create klien: {}", e);
            // Check for specific foreign key violations, e.g., if pk_id or bapas_id doesn't exist.
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_foreign_key_violation() {
                    return Err(StatusCode::BAD_REQUEST); // 400 Bad Request
                }
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


// --- READ ALL (with Pagination) ---
pub async fn get_all_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>, // <-- We now need the user's claims
    pagination: Query<Pagination>,
) -> Result<Json<Vec<Klien>>, StatusCode> {
    
    let offset = (pagination.page - 1) * pagination.limit;

    // --- NEW: DYNAMIC QUERY BUILDING BASED ON ROLE ---

    // Start with the base query.
      let mut query_builder: sqlx::QueryBuilder<Postgres> = sqlx::QueryBuilder::new(
        r#"
        SELECT
            id,
            tipe,
            nama,
            alamat,
            tempat_lahir,
            tanggal_lahir,
            jenis_kelamin,
            agama,
            pekerjaan,
            pendidikan_terakhir,
            bapas_id,
            kanwil_id,
            pk_id,
            created_at,
            updated_at,
            created_by,
            updated_by
        FROM klien
        "#
    );
    // Dynamically add a WHERE clause based on the user's role.
    match claims.role {
        crate::types::UserRole::SuperAdmin => {
            // SuperAdmin sees everyone. No WHERE clause is added.
        }

        crate::types::UserRole::AdminKanwil => {
            // AdminKanwil sees all clients within their kanwil_id.
            // The filter is applied on the JOINED bapas table.
            if let Some(kanwil_id) = claims.kanwil_id {
                query_builder.push(" WHERE b.kanwil_id = ");
                query_builder.push_bind(kanwil_id);
            } else {
                return Ok(Json(Vec::new())); // Security: return empty if misconfigured
            }
        }
        crate::types::UserRole::AdminBapas => {
            // AdminBapas sees all clients within their unit_kerja_id.
            if let Some(unit_kerja_id) = claims.unit_kerja_id {
                query_builder.push(" WHERE bapas_id = ");
                query_builder.push_bind(unit_kerja_id);
            } else {
                // This AdminBapas is not associated with any Bapas, which is an error state.
                // Return an empty list for security.
                return Ok(Json(Vec::new()));
            }
        }
        crate::types::UserRole::Pegawai => {
            // A Pegawai only sees clients directly assigned to them.
            query_builder.push(" WHERE pk_id = ");
            query_builder.push_bind(claims.sub); // claims.sub is the user's own ID
        }
    }

    // Add the final ordering and pagination to the query.
    query_builder.push(" ORDER BY nama LIMIT ");
    query_builder.push_bind(pagination.limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    // ---------------------------------------------------

    // Build the final query
    let query = query_builder.build_query_as::<Klien>();

    let klien_list = query
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch all klien: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(klien_list))
}

// --- READ ONE ---
pub async fn get_klien_by_id(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<Klien>, StatusCode> {
    let klien = sqlx::query_as("SELECT * FROM klien WHERE id = $1")
        .bind(id).fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if !AuthContext::new(&pool, &klien, &claims).is_authorized().await? {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(Json(klien))
}

// --- UPDATE ---
pub async fn update_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateKlien>,
) -> Result<Json<Klien>, StatusCode> {
    let existing_klien = sqlx::query_as("SELECT * FROM klien WHERE id = $1")
        .bind(id).fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    if !AuthContext::new(&pool, &existing_klien, &claims).is_authorized().await? {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let updated_by_id = claims.sub;

    let result = sqlx::query_as(
        r#"
        UPDATE klien SET
            tipe = COALESCE($1, tipe),
            nama = COALESCE($2, nama),
            alamat = COALESCE($3, alamat),
            tempat_lahir = COALESCE($4, tempat_lahir),
            tanggal_lahir = COALESCE($5, tanggal_lahir),
            jenis_kelamin = COALESCE($6, jenis_kelamin),
            agama = COALESCE($7, agama),
            pekerjaan = COALESCE($8, pekerjaan),
            pendidikan_terakhir = COALESCE($9, pendidikan_terakhir),
            bapas_id = COALESCE($10, bapas_id),
            pk_id = COALESCE($11, pk_id),
            online_akses = COALESCE($12, online_akses),
            pengulangan = COALESCE($13, pengulangan),
            kewarganegaraan = COALESCE($14, kewarganegaraan),
            negara_asal = COALESCE($15, negara_asal),
            suku = COALESCE($16, suku),
            keterangan = COALESCE($17, keterangan),
            catatan = COALESCE($18, catatan),
            updated_by = $19
        WHERE id = $20
        RETURNING *
        "#,
    )
    .bind(payload.tipe).bind(payload.nama).bind(payload.alamat).bind(payload.tempat_lahir)
    .bind(payload.tanggal_lahir).bind(payload.jenis_kelamin).bind(payload.agama)
    .bind(payload.pekerjaan).bind(payload.pendidikan_terakhir).bind(payload.bapas_id)
    .bind(payload.pk_id).bind(payload.online_akses).bind(payload.pengulangan)
    .bind(payload.kewarganegaraan).bind(payload.negara_asal).bind(payload.suku)
    .bind(payload.keterangan).bind(payload.catatan).bind(updated_by_id)
    .bind(id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(updated_klien) => Ok(Json(updated_klien)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update klien: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// --- DELETE ---
pub async fn delete_klien(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> StatusCode {
    let klien_to_delete = match sqlx::query_as("SELECT * FROM klien WHERE id = $1")
        .bind(id).fetch_optional(&pool).await {
        Ok(Some(k)) => k,
        Ok(None) => return StatusCode::NOT_FOUND,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    if let Ok(false) | Err(_) = AuthContext::new(&pool, &klien_to_delete, &claims).is_authorized().await {
        return StatusCode::FORBIDDEN;
    }

    sqlx::query("DELETE FROM klien WHERE id = $1").bind(id).execute(&pool).await.ok();
    StatusCode::NO_CONTENT
}
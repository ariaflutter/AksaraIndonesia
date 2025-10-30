// File baru: src/klien/handlers_core.rs

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::auth::model::AuthenticatedUser;
use crate::types::UserRoleEnum;
use super::model_core::{CreateKlien, Klien, UpdateKlien};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetAllKlienParams {
    pub pk_id: Option<i32>,
    pub bapas_id: Option<i32>,
    pub kanwil_id: Option<i32>,
}

// --- READ ALL (dengan filter dan otorisasi) ---
pub async fn get_all_klien(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(params): Query<GetAllKlienParams>,
) -> Result<Json<Vec<Klien>>, StatusCode> {
    
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT id, tipe_klien, nama_klien, alamat_klien, tempat_lahir_klien, 
        tanggal_lahir_klien, jenis_kelamin_klien, agama_klien, pekerjaan_klien, 
        pendidikan_terakhir_klien, bapas_id, pk_id, kanwil_id, online_akses_klien, 
        pengulangan_klien, kewarganegaraan_klien, negara_asal_klien, suku_klien, 
        keterangan_klien, catatan_klien, created_at, updated_at, created_by, 
        updated_by, deleted_at
        FROM klien WHERE deleted_at IS NULL
        "#
    );

    // Terapkan filter berdasarkan role user untuk keamanan
    match user.role {
        UserRoleEnum::SuperAdmin => {
            // SuperAdmin bisa filter berdasarkan kanwil atau bapas apa pun
            if let Some(kanwil_id) = params.kanwil_id {
                query_builder.push(" AND kanwil_id = ").push_bind(kanwil_id);
            }
            if let Some(bapas_id) = params.bapas_id {
                query_builder.push(" AND bapas_id = ").push_bind(bapas_id);
            }
        },
        UserRoleEnum::AdminKanwil => {
            // AdminKanwil hanya bisa melihat data di dalam kanwilnya
            query_builder.push(" AND kanwil_id = ").push_bind(user.kanwil_id);
            if let Some(bapas_id) = params.bapas_id { // Bisa filter bapas di bawahnya
                query_builder.push(" AND bapas_id = ").push_bind(bapas_id);
            }
        },
        UserRoleEnum::AdminBapas => {
            // AdminBapas hanya bisa melihat data di dalam bapasnya
            query_builder.push(" AND bapas_id = ").push_bind(user.bapas_id);
        },
        UserRoleEnum::Pegawai => {
            // Pegawai hanya bisa melihat klien miliknya sendiri
            query_builder.push(" AND pk_id = ").push_bind(user.id);
        }
    }

    // Pegawai bisa filter klien miliknya sendiri, jadi ini tetap berlaku
    if let Some(pk_id) = params.pk_id {
        query_builder.push(" AND pk_id = ").push_bind(pk_id);
    }
    
    query_builder.push(" ORDER BY nama_klien");

    let klien_list = query_builder.build_query_as::<Klien>()
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch klien list: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(klien_list))
}


// --- CREATE ---
pub async fn create_klien(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(payload): Json<CreateKlien>,
) -> Result<Json<Klien>, StatusCode> {
    // Otorisasi: AdminBapas dan Pegawai bisa membuat klien di wilayahnya. SuperAdmin bisa di mana saja.
    // Trigger di DB akan menangani sinkronisasi bapas_id dan kanwil_id dari pk_id
    
    // Kita perlu memeriksa apakah pk_id yang diinput valid untuk user yang membuat
    let target_pk_bapas_id = sqlx::query_scalar!("SELECT bapas_id FROM users WHERE id = $1", payload.pk_id)
        .fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .flatten(); // flatten Option<Option<i32>> to Option<i32>

    if target_pk_bapas_id.is_none() {
        return Err(StatusCode::BAD_REQUEST); // PK tidak ditemukan
    }
    
    match user.role {
        UserRoleEnum::AdminBapas if user.bapas_id != target_pk_bapas_id => {
            return Err(StatusCode::FORBIDDEN); // AdminBapas hanya boleh menugaskan PK di bapasnya
        }
        UserRoleEnum::Pegawai if user.id != payload.pk_id => {
            return Err(StatusCode::FORBIDDEN); // Pegawai hanya boleh membuat klien untuk dirinya sendiri
        }
        _ => {} // SuperAdmin dan AdminKanwil (jika diizinkan) boleh
    }


    let new_klien = sqlx::query_as!(
        Klien,
        r#"
        INSERT INTO klien (
            tipe_klien, nama_klien, alamat_klien, tempat_lahir_klien, tanggal_lahir_klien,
            jenis_kelamin_klien, agama_klien, pekerjaan_klien, pendidikan_terakhir_klien,
            pk_id, online_akses_klien, pengulangan_klien, kewarganegaraan_klien,
            negara_asal_klien, suku_klien, keterangan_klien, catatan_klien, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $18)
        RETURNING
            id, tipe_klien AS "tipe_klien: _", nama_klien, alamat_klien, tempat_lahir_klien, 
            tanggal_lahir_klien, jenis_kelamin_klien AS "jenis_kelamin_klien: _", agama_klien, pekerjaan_klien AS "pekerjaan_klien: _", 
            pendidikan_terakhir_klien AS "pendidikan_terakhir_klien: _", bapas_id, pk_id, kanwil_id, online_akses_klien, 
            pengulangan_klien, kewarganegaraan_klien AS "kewarganegaraan_klien: _", negara_asal_klien, suku_klien, 
            keterangan_klien, catatan_klien, created_at, updated_at, created_by, 
            updated_by, deleted_at
        "#,
        payload.tipe_klien as _,
        payload.nama_klien,
        payload.alamat_klien,
        payload.tempat_lahir_klien,
        payload.tanggal_lahir_klien,
        payload.jenis_kelamin_klien as _,
        payload.agama_klien,
        payload.pekerjaan_klien as _,
        payload.pendidikan_terakhir_klien as _,
        payload.pk_id,
        payload.online_akses_klien.unwrap_or(false),
        payload.pengulangan_klien.unwrap_or(false),
        payload.kewarganegaraan_klien as _,
        payload.negara_asal_klien,
        payload.suku_klien,
        payload.keterangan_klien,
        payload.catatan_klien,
        user.id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create klien: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(new_klien))
}


// --- GET BY ID, UPDATE, DELETE ---
// Handler ini akan dilindungi oleh middleware otorisasi `authorize_klien_access`
// yang sudah kita rancang. Jadi, tidak perlu cek role di dalam handler.

pub async fn get_klien_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Klien>, StatusCode> {
    let klien = sqlx::query_as!(
        Klien,
        r#"
        SELECT
            id, tipe_klien AS "tipe_klien: _", nama_klien, alamat_klien, tempat_lahir_klien, 
            tanggal_lahir_klien, jenis_kelamin_klien AS "jenis_kelamin_klien: _", agama_klien, pekerjaan_klien AS "pekerjaan_klien: _", 
            pendidikan_terakhir_klien AS "pendidikan_terakhir_klien: _", bapas_id, pk_id, kanwil_id, online_akses_klien, 
            pengulangan_klien, kewarganegaraan_klien AS "kewarganegaraan_klien: _", negara_asal_klien, suku_klien, 
            keterangan_klien, catatan_klien, created_at, updated_at, created_by, 
            updated_by, deleted_at
        FROM klien WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&pool)
    .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(klien))
}

pub async fn update_klien(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateKlien>,
) -> Result<Json<Klien>, StatusCode> {
   let updated_klien = sqlx::query_as!(
    Klien,
    r#"
    UPDATE klien SET
        tipe_klien = COALESCE($1, tipe_klien),
        nama_klien = COALESCE($2, nama_klien),
        alamat_klien = COALESCE($3, alamat_klien),
        tempat_lahir_klien = COALESCE($4, tempat_lahir_klien),
        tanggal_lahir_klien = COALESCE($5, tanggal_lahir_klien),
        jenis_kelamin_klien = COALESCE($6, jenis_kelamin_klien),
        agama_klien = COALESCE($7, agama_klien),
        pekerjaan_klien = COALESCE($8, pekerjaan_klien),
        pendidikan_terakhir_klien = COALESCE($9, pendidikan_terakhir_klien),
        bapas_id = COALESCE($10, bapas_id),
        pk_id = COALESCE($11, pk_id),
        kanwil_id = COALESCE($12, kanwil_id),
        online_akses_klien = COALESCE($13, online_akses_klien),
        pengulangan_klien = COALESCE($14, pengulangan_klien),
        kewarganegaraan_klien = COALESCE($15, kewarganegaraan_klien),
        negara_asal_klien = COALESCE($16, negara_asal_klien),
        suku_klien = COALESCE($17, suku_klien),
        keterangan_klien = COALESCE($18, keterangan_klien),
        catatan_klien = COALESCE($19, catatan_klien),
        updated_by = $20,
        updated_at = NOW()
    WHERE id = $21 AND deleted_at IS NULL
    RETURNING
        id, 
        tipe_klien AS "tipe_klien: _", 
        nama_klien, 
        alamat_klien, 
        tempat_lahir_klien, 
        tanggal_lahir_klien, 
        jenis_kelamin_klien AS "jenis_kelamin_klien: _", 
        agama_klien, 
        pekerjaan_klien AS "pekerjaan_klien: _", 
        pendidikan_terakhir_klien AS "pendidikan_terakhir_klien: _", 
        bapas_id, 
        pk_id, 
        kanwil_id, 
        online_akses_klien, 
        pengulangan_klien, 
        kewarganegaraan_klien AS "kewarganegaraan_klien: _", 
        negara_asal_klien, 
        suku_klien, 
        keterangan_klien, 
        catatan_klien, 
        created_at, 
        updated_at, 
        created_by, 
        updated_by, 
        deleted_at
    "#,
    payload.tipe_klien as _,
    payload.nama_klien,
    payload.alamat_klien,
    payload.tempat_lahir_klien,
    payload.tanggal_lahir_klien,
    payload.jenis_kelamin_klien as _,
    payload.agama_klien,
    payload.pekerjaan_klien as _,
    payload.pendidikan_terakhir_klien as _,
    payload.bapas_id,
    payload.pk_id,
    payload.kanwil_id,
    payload.online_akses_klien,
    payload.pengulangan_klien,
    payload.kewarganegaraan_klien as _,
    payload.negara_asal_klien,
    payload.suku_klien,
    payload.keterangan_klien,
    payload.catatan_klien,
    user.id,
    id
)
.fetch_one(&pool)
.await?;

    .fetch_optional(&pool)
    .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(updated_klien))
}

pub async fn delete_klien(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> StatusCode {
    let result = sqlx::query!(
        "UPDATE klien SET deleted_at = NOW(), updated_by = $1 WHERE id = $2",
        user.id, id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
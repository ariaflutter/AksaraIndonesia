// File baru: src/auth/authorization.rs

use sqlx::PgPool;
use crate::auth::model::AuthenticatedUser;
use crate::types::UserRoleEnum;

/// Merepresentasikan "kepemilikan" dari sebuah resource (misalnya Klien, Bapas, User).
#[derive(Default, Debug)]
pub struct ResourceOwnership {
    pub pk_id: Option<i32>,
    pub bapas_id: Option<i32>,
    pub kanwil_id: Option<i32>,
}

/// Fungsi inti yang memeriksa izin berdasarkan aturan hierarkis.
pub fn check_permission(
    user: &AuthenticatedUser,
    resource: &ResourceOwnership,
) -> bool {
    match user.role {
        UserRoleEnum::SuperAdmin => true,

        UserRoleEnum::AdminKanwil => {
            user.kanwil_id.is_some()
                && resource.kanwil_id.is_some()
                && user.kanwil_id == resource.kanwil_id
        }

        UserRoleEnum::AdminBapas => {
            user.bapas_id.is_some()
                && resource.bapas_id.is_some()
                && user.bapas_id == resource.bapas_id
        }
        
        // Aturan standar: Pegawai hanya bisa akses klien yang ditugaskan langsung
        UserRoleEnum::Pegawai => {
            resource.pk_id.is_some() && Some(user.id) == resource.pk_id
        }
    }
}

/// Helper untuk mengambil data kepemilikan Klien dari database.
pub async fn get_klien_ownership(pool: &PgPool, klien_id: i32) -> Result<Option<ResourceOwnership>, sqlx::Error> {
    sqlx::query!(
        r#"
        SELECT pk_id, bapas_id, kanwil_id
        FROM klien
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        klien_id
    )
    .fetch_optional(pool)
    .await
    .map(|maybe_row| {
        maybe_row.map(|row| ResourceOwnership {
            pk_id: Some(row.pk_id),
            bapas_id: Some(row.bapas_id),
            kanwil_id: row.kanwil_id,
        })
    })
}
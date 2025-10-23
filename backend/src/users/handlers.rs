// src/users/handlers.rs
use axum::{extract::Extension, http::StatusCode, Json};
use sqlx::PgPool;
use super::model::User;

pub async fn get_all_users(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let query = "SELECT * FROM users ORDER BY nama";

    let users = sqlx::query_as::<_, User>(query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch users: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(users))
}
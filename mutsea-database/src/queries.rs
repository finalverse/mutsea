//! mutsea-database/src/queries.rs
//! Database query implementations for different backends

use crate::{
    backends::{DatabaseBackend, DatabasePool},
    error::{DatabaseError, DatabaseResult},
    models::AssetMetadata,
};
use mutsea_core::{UserAccount, UserId, Asset, AssetId, RegionInfo, RegionId};

/// User queries
pub struct UserQueries {
    backend: DatabaseBackend,
}

impl UserQueries {
    pub fn new(backend: DatabaseBackend) -> Self {
        Self { backend }
    }

    pub async fn create(&self, pool: &DatabasePool, account: &UserAccount) -> DatabaseResult<()> {
        match pool {
            DatabasePool::PostgreSQL(pool) => {
                sqlx::query!(
                    "INSERT INTO users (id, first_name, last_name, email, password_hash, created, user_level, user_flags) 
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                    account.user_id.as_uuid(),
                    account.first_name,
                    account.last_name,
                    account.email,
                    account.password_hash,
                    account.created,
                    account.user_level,
                    account.user_flags
                ).execute(pool).await?;
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query!(
                    "INSERT INTO users (id, first_name, last_name, email, password_hash, created, user_level, user_flags) 
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                    account.user_id.as_uuid().as_bytes(),
                    account.first_name,
                    account.last_name,
                    account.email,
                    account.password_hash,
                    account.created,
                    account.user_level,
                    account.user_flags
                ).execute(pool).await?;
            }
            DatabasePool::SQLite(pool) => {
                sqlx::query!(
                    "INSERT INTO users (id, first_name, last_name, email, password_hash, created, user_level, user_flags) 
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    account.user_id.to_string(),
                    account.first_name,
                    account.last_name,
                    account.email,
                    account.password_hash,
                    account.created,
                    account.user_level,
                    account.user_flags
                ).execute(pool).await?;
            }
        }
        Ok(())
    }

    pub async fn get_by_id(&self, pool: &DatabasePool, user_id: UserId) -> DatabaseResult<Option<UserAccount>> {
        match pool {
            DatabasePool::PostgreSQL(pool) => {
                let row = sqlx::query!(
                    "SELECT id, first_name, last_name, email, password_hash, created, last_login, user_level, user_flags, user_title 
                     FROM users WHERE id = $1",
                    user_id.as_uuid()
                ).fetch_optional(pool).await?;

                if let Some(row) = row {
                    Ok(Some(UserAccount {
                        user_id: UserId::from_uuid(row.id),
                        first_name: row.first_name,
                        last_name: row.last_name,
                        email: row.email,
                        password_hash: row.password_hash,
                        created: row.created,
                        last_login: row.last_login,
                        user_level: row.user_level,
                        user_flags: row.user_flags,
                        user_title: row.user_title,
                    }))
                } else {
                    Ok(None)
                }
            }
            DatabasePool::MySQL(pool) => {
                let row = sqlx::query!(
                    "SELECT id, first_name, last_name, email, password_hash, created, last_login, user_level, user_flags, user_title 
                     FROM users WHERE id = ?",
                    user_id.as_uuid().as_bytes()
                ).fetch_optional(pool).await?;

                if let Some(row) = row {
                    let uuid_bytes: [u8; 16] = row.id.try_into()
                        .map_err(|_| DatabaseError::InvalidFormat("Invalid UUID format".to_string()))?;
                    Ok(Some(UserAccount {
                        user_id: UserId::from_uuid(uuid::Uuid::from_bytes(uuid_bytes)),
                        first_name: row.first_name,
                        last_name: row.last_name,
                        email: row.email,
                        password_hash: row.password_hash,
                        created: row.created,
                        last_login: row.last_login,
                        user_level: row.user_level,
                        user_flags: row.user_flags,
                        user_title: row.user_title,
                    }))
                } else {
                    Ok(None)
                }
            }
            DatabasePool::SQLite(pool) => {
                let row = sqlx::query!(
                    "SELECT id, first_name, last_name, email, password_hash, created, last_login, user_level, user_flags, user_title 
                     FROM users WHERE id = ?1",
                    user_id.to_string()
                ).fetch_optional(pool).await?;

                if let Some(row) = row {
                    let user_id = uuid::Uuid::parse_str(&row.id)
                        .map_err(|_| DatabaseError
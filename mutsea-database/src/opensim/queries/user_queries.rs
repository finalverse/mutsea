// src/opensim/queries/user_queries.rs
//! User account related database queries

use super::super::{schema::*, models::*};
use crate::{DatabaseManager, Result};

impl DatabaseManager {
    /// Insert a new user account
    pub async fn insert_user_account(&self, user: &UserAccount) -> Result<()> {
        let backend = self.get_backend().await?;
        let query = include_str!("../../sql/opensim/insert_user_account.sql");

        backend
            .execute(
                query,
                &[
                    &user.principal_id,
                    &user.scope_id,
                    &user.first_name,
                    &user.last_name,
                    &user.email,
                    &user.created,
                    &user.user_level,
                    &user.user_flags,
                    &user.active,
                ],
            )
            .await?;

        Ok(())
    }

    /// Get user account by principal ID
    pub async fn get_user_account(&self, principal_id: &str) -> Result<Option<UserAccount>> {
        let backend = self.get_backend().await?;
        let query = include_str!("../../sql/opensim/select_user_account.sql");

        let row = backend.query_optional(query, &[&principal_id]).await?;

        if let Some(row) = row {
            Ok(Some(UserAccount {
                principal_id: row.get("principal_id")?,
                scope_id: row.get("scope_id")?,
                first_name: row.get("first_name")?,
                last_name: row.get("last_name")?,
                email: row.get("email").ok(),
                service_urls: row.get("service_urls").ok(),
                created: row.get("created")?,
                user_level: row.get("user_level")?,
                user_flags: row.get("user_flags")?,
                user_title: row.get("user_title").ok(),
                active: row.get("active")?,
            }))
        } else {
            Ok(None)
        }
    }
}

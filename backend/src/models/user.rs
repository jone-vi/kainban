use anyhow::Context;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::token;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub api_token_hash: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserWithToken {
    pub user: User,
    pub token: String,
}

impl User {
    pub async fn create(pool: &MySqlPool, new: CreateUser) -> anyhow::Result<UserWithToken> {
        let id = Uuid::now_v7();
        let token = token::generate_token();
        let token_hash = token::hash_token(&token);

        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, api_token_hash)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(new.name)
        .bind(new.email)
        .bind(token_hash.as_slice())
        .execute(pool)
        .await
        .context("inserting user")?;

        let user = Self::get_user(pool, id).await?;

        Ok(UserWithToken { user, token })
    }

    pub async fn get_user(pool: &MySqlPool, id: Uuid) -> anyhow::Result<Self> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .context("loading user")?;

        Ok(user)
    }

    pub async fn find_by_token(pool: &MySqlPool, token: &str) -> anyhow::Result<Option<Self>> {
        let token_hash = token::hash_token(token);

        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE api_token_hash = ?
            LIMIT 1
            "#
        )
        .bind(token_hash.as_slice())
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}

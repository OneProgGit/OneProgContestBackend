//! [`PostgresDatabase`]

use std::sync::Arc;

use anyhow::Ok;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::{
    db::Database,
    models::{post::Post, user::NewDbUser, user::User},
};

/// Postgresql database using Sqlx
#[derive(Clone)]
pub struct PostgresDatabase {
    pool: PgPool,
}

impl Database for PostgresDatabase {
    async fn new(url: String) -> anyhow::Result<Arc<Self>> {
        let pool = PgPoolOptions::new().connect(&url).await?;
        Ok(Arc::new(Self { pool }))
    }

    async fn create_user(&self, user: NewDbUser) -> anyhow::Result<()> {
        sqlx::query(
            "
            INSERT INTO users (username, hashed_password)
            VALUES ($1, $2)
            ",
        )
        .bind(user.username)
        .bind(user.hashed_password)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        let user = sqlx::query_as("SELECT * FROM users WHERE id=$1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn get_posts(&self) -> anyhow::Result<Vec<Post>> {
        let posts = sqlx::query_as("SELECT * FROM posts")
            .fetch_all(&self.pool)
            .await?;
        Ok(posts)
    }
}

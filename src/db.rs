//! [`Database`]

#![allow(async_fn_in_trait)]

use std::sync::Arc;

use uuid::Uuid;

use crate::models::{post::Post, user::NewDbUser, user::User};

pub mod postgres;

/// Used for all database operations.
pub trait Database: Send + Sync + Clone {
    /// Creates a new Database instance and connects to database.
    /// # Example
    /// ```
    /// let db = SomeDatabase::new().await?;
    /// ```
    async fn new(url: String) -> anyhow::Result<Arc<Self>>;

    /// Creates a new user.
    /// # Example
    /// ```
    /// let new_user = NewDbUser { username: "OneProg", hashed_password: "******" };
    /// db.create_user(new_user).await?;
    /// ```
    async fn create_user(&self, user: NewDbUser) -> anyhow::Result<()>;

    /// Gets user by given id.
    /// # Example
    /// ```
    /// let user = db.get_user_by_id(Uuid::now_v7()).await?;
    /// println!("Your username: {}", user.username);
    /// ```
    async fn get_user_by_id(&self, id: Uuid) -> anyhow::Result<User>;

    /// Gets all posts, which showed in main page.
    /// # Example
    /// ```
    /// let posts = db.get_posts()?;
    /// for post in posts {
    ///     println!("Post author: {}", post.author);
    /// }
    /// ```
    async fn get_posts(&self) -> anyhow::Result<Vec<Post>>;
}

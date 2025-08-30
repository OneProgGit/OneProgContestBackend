//! [`Database`]

#![allow(async_fn_in_trait)]

use std::sync::Arc;

use uuid::Uuid;

use crate::models::{
    post::{NewDbPost, Post},
    user::{NewDbUser, User},
};

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
    /// let new_user = NewDbUser { username: "OneProg", hashed_password: "******".into() };
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

    /// Gets user by given username;
    /// # Example
    /// ```
    /// let user = NewDbUser { username: "OneProg", hashed_password: "******".into() };
    /// db.create_user(user).await?;
    /// assert_eq!("******".into(), db.get_user_by_username(user.username).unwrap().hashed_password);
    /// ```
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<User>;

    /// Creates a new post, which showed in main page.
    /// # Example
    /// ```
    /// let post = NewDbPost { author: "OneProg", title: "Lorem ipsum", content: "Lorem ipsum" };
    /// db.create_post(post).await?;
    /// ```
    async fn create_post(&self, post: NewDbPost) -> anyhow::Result<()>;

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

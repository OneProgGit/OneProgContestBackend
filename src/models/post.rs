//! [`Post`]
//! [`NewPost`]

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

/// Defines post data. Used for both getting and creating posts.
/// # Example
/// ```
/// let post = Post { id: 52, author: "OneProg", title: "Lorem ipsum", content: "Lorem ipsum" };
/// ```
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Post {
    /// Unique post id
    pub id: Uuid,
    /// Username of post author
    pub author: String,
    /// Post title
    pub title: String,
    /// Post content
    pub content: String,
}

/// Defines a new post data. Used for frontend requests.
#[derive(Serialize, Deserialize)]
pub struct PostData {
    /// Author's JWT token
    pub token: String,
    /// Post title
    pub title: String,
    /// Post content
    pub content: String,
}

/// Defines a new post for database.
#[derive(Serialize, Debug)]
pub struct NewDbPost {
    /// Post author
    pub author: String,
    /// Post title
    pub title: String,
    /// Post content
    pub content: String,
}

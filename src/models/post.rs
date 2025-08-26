//! [`Post`]
//! # Example
//! ```
//! let post = Post { id: 52, author: "OneProg", title: "Lorem ipsum", content: "Lorem ipsum" };
//! ```

#![allow(dead_code)] // TODO: remove this line

use serde::{Deserialize, Serialize};

/// Defines post data. Used for both getting and creating posts.
#[derive(Serialize, Deserialize)]
pub struct Post {
    /// Unique post id
    pub id: i32,
    /// Username of post author
    pub author: String,
    /// Post title
    pub title: String,
    /// Post content
    pub content: String,
}

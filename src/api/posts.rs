//! Defines methods for getting and creating posts

use axum::Json;

use crate::models::post::Post;

/// Used when /posts get request called. Gets all posts from database and returns them.
pub async fn get_posts() -> Json<Vec<Post>> {
    Json(vec![
        Post {
            id: 0,
            author: "OneProg".into(),
            title: "Test".into(),
            content: "It works".into(),
        },
        Post {
            id: 1,
            author: "OneProg".into(),
            title: "Another".into(),
            content: "Another it works".into(),
        },
    ])
}

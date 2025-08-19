use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: i32,
    author: &'static str,
    title: &'static str,
    content: &'static str,
}

pub async fn get_posts() -> Json<Vec<Post>> {
    Json(vec![
        Post {
            id: 0,
            author: "OneProg",
            title: "Test",
            content: "It works",
        },
        Post {
            id: 1,
            author: "OneProg",
            title: "Another",
            content: "Another it works",
        },
    ])
}

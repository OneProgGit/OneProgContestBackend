//! Defines methods for getting and creating posts

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{Value, json};

use crate::{
    AppStateType,
    db::Database,
    jwt::decode_jwt,
    models::post::{NewDbPost, PostData},
};

/// Used when /posts post request called. Creates a new post if user is an admin
/// # Errors
/// Returns an error if failed to validate JWT,
pub async fn create_post(
    State(state): State<AppStateType>,
    Json(post): Json<PostData>,
) -> impl IntoResponse {
    let secret = dotenvy::var("JWT_SECRET").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Server error" })),
        )
    })?;
    let claims = decode_jwt(&post.token, &secret).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
    })?;
    let user = state.db.get_user_by_id(claims.id).await.map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
    })?;
    if !user.admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "You are not admin" })),
        ));
    }
    let db_new_post = NewDbPost {
        author: user.username,
        title: post.title,
        content: post.content,
    };
    state.db.create_post(db_new_post).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Server error" })),
        )
    })?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Post created successfully" })),
    ))
}

/// Used when /posts get request called. Gets all posts from database and returns them.
/// # Errors
/// Returns an error if failed to get posts from database
pub async fn get_posts(
    State(state): State<AppStateType>,
) -> anyhow::Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let posts = state.db.get_posts().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Server error" })),
        )
    })?;
    Ok((StatusCode::OK, Json(posts)))
}

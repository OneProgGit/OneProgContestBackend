//! Defines methods for getting and creating posts

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::{AppStateType, db::Database, models::post::Post};

/// Used when /posts get request called. Gets all posts from database and returns them.
/// # Errors
/// Returns an error if failed to get posts from database
pub async fn get_posts(
    State(state): State<AppStateType>,
) -> anyhow::Result<Json<Vec<Post>>, StatusCode> {
    state
        .db
        .get_posts()
        .await
        .map_or(Err(StatusCode::INTERNAL_SERVER_ERROR), |posts| {
            Ok(Json(posts))
        })
}

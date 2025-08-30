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
    middleware::auth::Auth,
    models::post::{NewDbPost, PostData},
};

/// Used when /posts post request called. Creates a new post if user is an admin
/// # Errors
/// Returns an error if failed to validate JWT,
pub async fn create_post(
    State(state): State<AppStateType>,
    Auth(user): Auth,
    Json(post): Json<PostData>,
) -> impl IntoResponse {
    if !user.admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(
                json!({ "error": "Для создания поста необходимы привилегии администратора, коих у Вас нет" }),
            ),
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
            Json(json!({ "error": "Не удалось создать пост" })),
        )
    })?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Пост создан успешно" })),
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

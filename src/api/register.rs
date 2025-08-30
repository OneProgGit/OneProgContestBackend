//! Defines method for registering a new user.
//!
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{Value, json};

use crate::{
    AppStateType,
    crypt::hash_password,
    db::Database,
    models::user::{LogInData, NewDbUser},
};

/// Used when /users post request called. Registers a new user
/// # Errors
/// Returns an error when failed to hash password or create user in database
pub async fn register(
    State(state): State<AppStateType>,
    Json(user): Json<LogInData>,
) -> anyhow::Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let hashed_password = hash_password(&user.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Не удалось получить хэш пароля" })),
        )
    })?;

    let new_user = NewDbUser {
        username: user.username,
        hashed_password,
    };

    state.db.create_user(new_user).await.map_err(|e| {
        let text = e.to_string();
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": if text.contains("повторяющееся") {
                "Пользователь с таким именем уже зарегистрирован"
            } else {
                "Ошибка создания пользователя"
            }})),
        )
    })?;

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Пользователь зарегистрирован успешно" })),
    ))
}

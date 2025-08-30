#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

//! Defines method for logging into account.

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Duration, Utc};
use serde_json::json;

use crate::{
    AppStateType,
    crypt::verify_password,
    db::Database,
    jwt::create_jwt,
    models::{jwt::JwtClaims, user::LogInData},
};

/// Used when /login post request called. Loggins into account
/// # Errors
/// Returns an error if failed to check password, the password is invalid, or failed to create JWT token.
pub async fn login(
    State(state): State<AppStateType>,
    Json(user): Json<LogInData>,
) -> impl IntoResponse {
    let expected_user = state
        .db
        .get_user_by_username(&user.username)
        .await
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Пользователя с таким именем не существует" })),
            )
        })?;

    let is_valid_password = verify_password(&expected_user.hashed_password, &user.password)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Не удалось проверить пароль" })),
            )
        })?;

    if !is_valid_password {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Неверный пароль" })),
        ));
    }

    let claims = JwtClaims {
        id: expected_user.id,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    let secret = dotenvy::var("JWT_SECRET").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Не удалось создать JWT токен" })),
        )
    })?;

    let token = create_jwt(&claims, &secret).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Не удалось создать JWT токен" })),
        )
    })?;

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Успешный вход в аккаунт", "token": token })),
    ))
}

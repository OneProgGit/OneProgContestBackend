//! [`Auth`]

use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use serde_json::{Value, json};

use crate::{AppStateType, db::Database, jwt::decode_jwt, models::user::User};

/// Defines authentication middleware
pub struct Auth(pub User);

impl FromRequestParts<AppStateType> for Auth {
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppStateType,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Отсутствует JWT токен" })),
                )
            })?;

        let secret = dotenvy::var("JWT_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Не удалось проверить JWT токен" })),
            )
        })?;
        let claims = decode_jwt(token, &secret).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Невалидный JWT токен" })),
            )
        })?;
        let user = state.db.get_user_by_id(claims.id).await.map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Невалидный JWT токен" })),
            )
        })?;

        Ok(Self(user))
    }
}

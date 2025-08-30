//! Defines method for getting user data

use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};

use crate::{AppStateType, middleware::auth::Auth};

/// Used when /user get request called. Gets user data
pub async fn get_user_data(
    State(_): State<AppStateType>,
    Auth(user): Auth,
) -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!(user)))
}

//! Defines method for registering a new user.

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

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
) -> StatusCode {
    if let Ok(hashed_password) = hash_password(&user.password) {
        let new_user = NewDbUser {
            username: user.username,
            hashed_password,
        };

        state
            .db
            .create_user(new_user)
            .await
            .map_or(StatusCode::BAD_REQUEST, |()| StatusCode::OK)
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

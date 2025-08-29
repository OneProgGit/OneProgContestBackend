//! Defines method for registering a new user.

use axum::Json;

use crate::{crypt::hash_password, models::new_user::NewUser};

/// Used when /users post request called. Registers a new user
pub async fn register(Json(user): Json<NewUser>) -> Json<String> {
    let hashed_password = hash_password(user.password)?;
}

//! [`User`]

use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

/// Defines user. Used for all user operations such as register or logging in, getting profile.
/// # Example
/// ```
/// let user = User { id: 52, username: "OneProg", password_hash: "******" };
/// println!("{}", user.username);
/// ```
#[derive(Deserialize, Debug, FromRow)]
pub struct User {
    /// Unique user id
    pub id: Uuid,
    /// Unique username
    pub username: String,
    /// User's hashed password (not raw!)
    pub hashed_password: String,
}

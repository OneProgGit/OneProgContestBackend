//! [`User`]
//! [`RequestUserData`]
//! [`NewDbUser`]

use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

/// Defines user. Used for all user operations such as register or logging in, getting profile.
/// # Example
/// ```
/// let user = User { id: 52, username: "OneProg", password_hash: "******".into() };
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
    /// Is user an admin?
    pub admin: bool,
}

/// Defines a user used for frontend requests
/// # Example
/// ```
/// const res = await fetch(apiUrl + "/user", {
///     method: "POST",
///     headers: {
///         "Content-Type": "application/json"
///     },
///     body: JSON.stringify({
///         username: "OneProg",
///         password: "123456"    
///     })
/// })
/// ```

#[derive(Deserialize)]
pub struct LogInData {
    /// User's name
    pub username: String,
    /// User's password
    pub password: String,
}

/// Defines a new user to create in database
/// # Example
/// ```
/// let new_user = NewUser { username: "OneProg", hashed_password: "******".into() };
/// db.create_user(new_user).await?;
/// ```
pub struct NewDbUser {
    /// User's name
    pub username: String,
    /// User's password which has already hashed
    pub hashed_password: String,
}

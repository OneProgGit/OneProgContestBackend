//! [`User`]
//! # Example
//! ```
//! let user = User { id: 52, username: "OneProg", password_hash: "******" };
//! println!("{}", user.username);
//! ```

#![allow(dead_code)] // TODO: remove this line

use serde::Deserialize;

/// Defines user. Used for all user operations such as register or logging in, getting profile.
#[derive(Deserialize)]
pub struct User {
    /// Unique user id
    pub id: u64, // TODO: replace to UUID
    /// Unique username
    pub username: String,
    /// User's hashed password (not raw!)
    pub hashed_password: String,
}

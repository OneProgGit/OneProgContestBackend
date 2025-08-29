//! [`NewUser`]

/// Defines a new user used for frontend requests
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
pub struct NewUser {
    pub username: String,
    pub password: String,
}

/// Defines a new user to create in database
/// # Example
/// ```
/// let new_user = NewUser { username: "OneProg", hashed_password: "******" };
/// db.create_user(new_user).await?;
/// ```
pub struct NewDbUser {
    pub username: String,
    pub hashed_password: String,
}

//! Hashed and verifies passwords.
//! # Example
//! ```
//! let pass: String = "123456".into();
//! let hashed = hash_password(pass);
//! assert!(verify_password(hashed, pass).unwrap_or(false));
//! ```

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

/// Hashes password using argon2
/// # Errors
/// Returns an error when argon2 operations failed
pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(hashed.to_string())
}

/// Verifies password using argon2
/// # Errors
/// Returns an error when argon2 operations failed
pub fn verify_password(excepted: &str, password: &str) -> anyhow::Result<bool> {
    let hash = &PasswordHash::new(excepted).map_err(|e| anyhow::anyhow!(e))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), hash).is_ok())
}

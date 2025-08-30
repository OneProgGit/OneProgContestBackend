//! Jwt provides methods for creating and validating authorization tokens

use anyhow::Ok;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::models::jwt::JwtClaims;

/// Creates JWT token.
/// # Example
/// ```
/// let claims = JwtClaims { username: String, exp: i64 };
/// let token = create_jwt(claims, "Hello, world!")?;
/// println!("Your token: {token}");
/// ```
/// # Errors
/// Returns an error when failed to create a token
pub fn create_jwt(claims: &JwtClaims, secret: &str) -> anyhow::Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

/// Validates and decodes JWT token.
/// # Example
/// ```
/// let token = "1234";
/// let claims = decode_jwt(token, "Hello, world!")?;
/// println!("Your id: {}", claims.id);
/// ```
/// # Errors
/// Returns an error when token is invalid or expired
pub fn decode_jwt(token: &str, secret: &str) -> anyhow::Result<JwtClaims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let claims = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(claims.claims)
}

//! [`JwtClaims`]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Jwt claims is data used to generate authorization token

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    /// User's id
    pub id: Uuid,
    /// Token expire time
    pub exp: i64,
}

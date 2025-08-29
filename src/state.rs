//! [`AppState`]

use std::sync::Arc;

use crate::db::Database;

/// Defines app state
/// # Example
/// ```
/// async fn foo(State(state): State<AppState>) -> anyhow::Result<()> {
///     state.db.create_user(NewUser { username: "OneProg", password: "123456" })?;
/// }
/// ```
#[derive(Clone)]
pub struct AppState<Db: Database> {
    /// Database reference
    pub db: Arc<Db>,
}

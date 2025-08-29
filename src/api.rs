//! Defines all API methods, like getting posts or registering user.
//! You should only use it in routing in main.rs file, like that:
//! ```
//! let app = Router::new(). /* ... */ .route("/foo", post(foo));
//! ```

pub mod posts;
pub mod register;

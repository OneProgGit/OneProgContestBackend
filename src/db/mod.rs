//! [`Database`]

use crate::models::{post::Post, user::User};

/// Used for all database operations.
pub trait Database {
    /// Creates a new Database instance and connects to database.
    /// # Example
    /// ```
    /// let db = SomeDatabase::new();
    /// ```
    fn new(url: String) -> Self;

    /// Creates a new user.
    /// # Example
    /// ```
    /// let user = User { id: 52, username: "OneProg", password_hash: "******" };
    /// db.create_user(user);
    /// ```
    fn create_user(&self, user: User);

    /// Gets user by given id.
    /// # Example
    /// ```
    /// let user = db.get_user_by_id(52);
    /// println!("Your username: {}", user.username);
    /// ```
    fn get_user_by_id(&self, id: u64) -> User;

    /// Gets all posts, which showed in main page.
    /// # Example
    /// ```
    /// let posts = db.get_posts();
    /// for post in posts {
    ///     println!("Post author: {}", post.author);
    /// }
    /// ```
    fn get_posts() -> Vec<Post>;
}

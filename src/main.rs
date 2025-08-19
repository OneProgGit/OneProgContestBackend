use axum::{Router, http::HeaderValue, routing::get};
use tower_http::cors::{Any, CorsLayer};

use crate::api::posts::get_posts;

mod api;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:5173"))
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new().route("/users", get(get_posts)).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to create tcp listener");
    axum::serve(listener, app).await.expect("Failed to serve");
}

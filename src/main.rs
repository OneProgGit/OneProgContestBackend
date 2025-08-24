//! This backend is a public API to interact with `OneProg Contest`

#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(rustdoc::all)]
#![deny(missing_docs)]
#![allow(clippy::multiple_crate_versions)]

use std::env;

use axum::{Router, http::HeaderValue, routing::get};
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};

use crate::api::posts::get_posts;

mod api;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let allowed_url = env::var("ALLOWED_URL").expect("ALLOWED_URL must be set");
    let host_url = env::var("HOST_URL").expect("HOST_URL must be set");

    let cors = CorsLayer::new()
        .allow_origin(
            HeaderValue::from_str(allowed_url.as_str()).expect("Failed to set header value"),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new().route("/users", get(get_posts)).layer(cors);

    let listener = tokio::net::TcpListener::bind(host_url)
        .await
        .expect("Failed to create tcp listener");
    axum::serve(listener, app).await.expect("Failed to serve");
}

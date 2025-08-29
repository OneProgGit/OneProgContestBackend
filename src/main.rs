//! This backend is a public API to interact with `OneProg Contest`

#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(rustdoc::all)]
#![deny(missing_docs)]
#![allow(clippy::multiple_crate_versions)]

use axum::{Router, http::HeaderValue, routing::get};
use dotenvy::dotenv;
use tokio::{net::TcpListener, signal};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    api::posts::get_posts,
    db::{Database, postgres::PostgresDatabase},
    state::AppState,
};

pub mod api;
pub mod crypt;
pub mod db;
pub mod models;
pub mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let allowed_url = dotenvy::var("ALLOWED_URL").unwrap_or_else(|_| {
        eprintln!("ALLOWED_URL not set: using default");
        "https://code.oneprog.org".into()
    });
    let host_url = dotenvy::var("HOST_URL").unwrap_or_else(|_| {
        eprintln!("HOST_URL not set: using default");
        "127.0.0.1:3000".into()
    });
    let db_url = dotenvy::var("DB_URL").expect("DB_URL must be set!");

    let cors = CorsLayer::new()
        .allow_origin(
            HeaderValue::from_str(allowed_url.as_str()).expect("Failed to set header value"),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let db = PostgresDatabase::new(db_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState { db };
    let router = Router::new()
        .route("/posts", get(get_posts))
        .layer(cors)
        .with_state(state);
    let app = Router::new().nest("/contest", router);

    let listener = TcpListener::bind(host_url)
        .await
        .expect("Failed to create tcp listener");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to serve app");
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
}

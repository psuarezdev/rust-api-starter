pub mod router;
pub mod schema;
pub mod users;
pub mod auth;
pub mod lib;

use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    println!("{}", env::var("APP_ENV").unwrap_or_default());

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ]);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let app = router::create_router().layer(cors);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

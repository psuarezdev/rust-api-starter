use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};
use tracing_subscriber::{FmtSubscriber, EnvFilter};
use axum::{http::Request, middleware::Next, response::Response};
use axum::body::Body;

pub mod router;
pub mod schema;
pub mod users;
pub mod auth;
pub mod lib;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE) // Set log level to TRACE for more detailed logs
        .with_env_filter(EnvFilter::new("axum=trace")) // Add axum logging filter
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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

    let app = router::create_router()
        .layer(cors)
        .layer(axum::middleware::from_fn(log_requests));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn log_requests(req: Request<Body>, next: Next) -> Response {
    info!("Received request: {} {}", req.method(), req.uri());
    let response = next.run(req).await;
    info!("Response with status: {}", response.status());
    response
}

use axum::Router;
use axum::routing::post;
use crate::auth;
use crate::lib::state::AppState;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::handlers::login))
        .route("/register", post(auth::handlers::register))
}
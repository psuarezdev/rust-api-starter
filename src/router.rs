use crate::auth::router::create_auth_router;
use crate::lib::{db, state};
use axum::Router;
use dotenvy::dotenv;
use std::env;

pub fn create_router() -> Router {
    dotenv().ok();

    // let environment = env::var("APP_ENV").unwrap_or_else(|_| "production".into());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let db_pool = db::create_pool(&database_url);
    
    let app_state = state::AppState { db_pool, jwt_secret };

    Router::new()
        .nest("/api/auth", create_auth_router())
        .with_state(app_state)
    /*if environment == "production" {
        let frontend_serve_dir =
            ServeDir::new("./static").not_found_service(ServeFile::new("./static/index.html"));
        router
            .nest_service("/static", ServeDir::new("./static"))
            .fallback_service(frontend_serve_dir)
    } else {
        router.route(
            "/",
            get(|| async { Html("Backend in development mode. Run the frontend separately.") }),
        )
    }*/
}


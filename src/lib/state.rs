use crate::lib::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub jwt_secret: String,
}

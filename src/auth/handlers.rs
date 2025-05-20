use crate::auth::requests::{LoginRequest, RegisterRequest};
use crate::lib::{errors::AppError, state::AppState};
use crate::users::models::NewUser;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use bcrypt::{verify, DEFAULT_COST};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let LoginRequest { email, password } = payload;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError(None))?;

    match crate::users::service::get_user_by_email(&mut conn, &email).await {
        Ok(user) => {
            let is_valid = verify(&password, &user.password)
                .map_err(|_| AppError::Unauthorized("Invalid credentials".into()))?;

            let token = crate::lib::jwt::create_jwt(&user.id.to_string(), &state.jwt_secret)
                .map_err(|_| AppError::InternalServerError(None))?;

            if is_valid {
                Ok(Json(serde_json::json!({
                    "user": user,
                    "token": token,
                })))
            } else {
                Err(AppError::Unauthorized("Invalid credentials".into()))
            }
        }
        Err(_) => Err(AppError::Unauthorized("Invalid credentials".into())),
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let RegisterRequest {
        first_name,
        last_name,
        email,
        password,
    } = payload;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError(None))?;
    
    let hashed_password = bcrypt::hash(&password, DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError(None))?;

    let new_user = NewUser {
        first_name,
        last_name,
        email,
        password: hashed_password,
    };

    match crate::users::service::create_user(&mut conn, new_user).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(AppError::Conflict("User already exists".into())),
    }
}

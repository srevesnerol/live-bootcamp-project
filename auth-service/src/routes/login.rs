use crate::domain::{AuthAPIError, Email, Password};
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let user_store = &state.user_store.read().await;

    let validate = user_store.validate_user(&email, &password).await;
    match validate {
        Err(_) => Err(AuthAPIError::IncorrectCredentials),
        Ok(_) => Ok(StatusCode::OK.into_response()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

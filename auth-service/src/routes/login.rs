use crate::domain::{AuthAPIError, Email, Password};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login(Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let _email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let _password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    Ok(StatusCode::OK.into_response())
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

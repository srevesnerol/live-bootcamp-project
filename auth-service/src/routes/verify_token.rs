use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::domain::AuthAPIError;
use crate::utils::auth::validate_token;

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token(Json(request): Json<VerifyTokenRequest>) -> Result<impl IntoResponse, AuthAPIError>  {
    match validate_token(&request.token).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}


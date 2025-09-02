use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login(Json(request): Json<LoginRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email.clone()) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(request.password.clone()) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;

    match user_store.validate_user(&email, &password).await {
        Err(_) => (jar, Err(AuthAPIError::IncorrectCredentials)),
        Ok(_) => {
            let auth_cookie = match generate_auth_cookie(&email) {
                Ok(cookie) => cookie,
                Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
            };

            let updated_jar = jar.add(auth_cookie);
            (updated_jar, Ok(StatusCode::OK.into_response()))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

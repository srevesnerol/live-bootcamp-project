use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie, CookieJar};

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let token = jar.get(JWT_COOKIE_NAME).map(|t| t.value().to_owned());

    match token {
        None => (jar, Err(AuthAPIError::MissingToken)),
        Some(t) => {
            let cookie = validate_token(&t).await;
            match cookie {
                Ok(_) => {
                    let updated_jar = jar.remove(JWT_COOKIE_NAME);
                    (updated_jar, Ok(StatusCode::OK)
                )},
                Err(_) => (jar, Err(AuthAPIError::InvalidToken)),
            }
        }
    }
}

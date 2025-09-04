use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    // Generate a valid JWT token for testing
    let email = auth_service::domain::email::Email::parse(get_random_email()).unwrap();
    let auth_cookie = auth_service::utils::auth::generate_auth_cookie(&email).unwrap();
    let cookie_value = auth_cookie.value();

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME, cookie_value
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice() {
    let app = TestApp::new().await;

    // Generate a valid JWT token for testing
    let email = auth_service::domain::email::Email::parse(get_random_email()).unwrap();
    let auth_cookie = auth_service::utils::auth::generate_auth_cookie(&email).unwrap();
    let cookie_value = auth_cookie.value();

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME, cookie_value
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.post_logout().await;
    let response2 = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response2.status().as_u16(), 400);
}

use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use crate::helpers::{get_random_email, TestApp};

use serde_json::json;

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    // First, create a user and login to get a real JWT token
    let random_email = get_random_email();
    
    let signup_body = json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });
    
    let response = app.post_signup(&signup_body).await;
    
    let login_body = json!({
        "email": random_email,
        "password": "password123"
    });
    
    let response = app.post_login(&login_body).await;
    
    // Extract the JWT token from the cookie
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    
    let jwt_token = auth_cookie.value();
    
    // Now test the verify_token endpoint with the real JWT
    let valid_request = json!({
        "token": jwt_token
    });

    let output = app.post_verify_token(&valid_request).await;
    assert_eq!(output.status().as_u16(), 200)
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let invalid_request = json!({
        "token": "invalid_token"
    });

    let output = app.post_verify_token(&invalid_request).await;
    assert_eq!(output.status().as_u16(), 401)
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;

    let banned_request = json!({
        "token": "banned_token"
    });
    {
        let mut banned_store = app.banned_token_store.write().await;
        banned_store.add_token("banned_token".to_string()).await.unwrap();
    }

    let output = app.post_verify_token(&banned_request).await;
    assert_eq!(output.status().as_u16(), 401)
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    // Send a malformed JWT token (not properly formatted JWT)
    let malformed_request = json!({
        "wrong": "malformed_token_not_jwt"
    });

    let output = app.post_verify_token(&malformed_request).await;
    assert_eq!(output.status().as_u16(), 422)
}

use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let malformed_request = serde_json::json!({
        "email": "email@example.com",
        "loginAttemptId": "123456",
    });

    let response = app.post_verify_2fa(&malformed_request).await;
    assert_eq!(response.status().as_u16(), 422)
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let invalid_request = serde_json::json!({
        "email": "invalid_email",
        "loginAttemptId": "invalid_login_attempt_id",
        "2FACode": "invalid_2fa_code"
    });

    let response = app.post_verify_2fa(&invalid_request).await;
    assert_eq!(response.status().as_u16(), 400)
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    let two_fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": response_body.login_attempt_id,
        "2FACode": "123456",
    });

    let response = app.post_verify_2fa(&two_fa_body).await;

    assert_eq!(response.status().as_u16(), 401);

}

// #[tokio::test]
// async fn should_return_401_if_old_code() {
//     // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail. 
//     todo!()
// }

#[tokio::test]
async fn should_return_200_if_correct_code() {

    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    let two_fa_code = app.two_fa_code_store.read().await.get_code(&Email::parse(random_email.clone()).unwrap()).await.expect("2FA code not found");

    let two_fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": response_body.login_attempt_id,
        "2FACode": two_fa_code.1.as_ref().to_string(),
    });

    let response = app.post_verify_2fa(&two_fa_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}
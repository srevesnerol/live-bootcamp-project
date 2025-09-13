use auth_service::domain::LoginAttemptId;

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

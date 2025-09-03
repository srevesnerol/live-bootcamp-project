use crate::helpers::{get_random_email, TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let login_body = serde_json::json!({
        "email": random_email
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "email": "bob@bob.com",
        "password": "passwordABC"
    });

    let response = app.post_login(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let input = [
        serde_json::json!({
            "email": "",
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
            "password": "",
        }),
        serde_json::json!({
            "email": "",
            "password": "",
        }),
        serde_json::json!({
            "email": "invalid_email",
            "password": "password123",
        }),
        // Password is to short
        serde_json::json!({
            "email": random_email,
            "password": "invalid",
        }),
    ];
    for i in input.iter() {
        let response = app.post_login(i).await;

        assert_eq!(response.status().as_u16(), 400);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;

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

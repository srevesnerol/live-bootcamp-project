use crate::helpers::{get_random_email, TestApp};

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

use crate::helpers::TestApp;

#[tokio::test]
async fn post_verify_2fa() {
    let app = TestApp::new().await;

    let response = app.post_verify_2_factor().await;

    assert_eq!(response.status().as_u16(), 200);
}

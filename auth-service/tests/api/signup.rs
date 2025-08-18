use crate::helpers::TestApp;

#[tokio::test]
async fn post_signup_route() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;

    assert_eq!(response.status().as_u16(), 200);
}

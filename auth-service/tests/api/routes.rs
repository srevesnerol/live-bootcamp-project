use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn post_verify_routes_works() {
    let app = TestApp::new().await;

    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn post_signup_route() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn post_login() {
    let app = TestApp::new().await;

    let response = app.post_login().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn post_logout() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn post_verify_2fa() {
    let app = TestApp::new().await;

    let response = app.post_verify_2_factor().await;

    assert_eq!(response.status().as_u16(), 200);
}

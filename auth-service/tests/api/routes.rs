use crate::helpers::TestApp;
use serde_json::json;

// Tokio's test macro is used to run the test in an async enviornment.
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_returns_200_for_valid_post_request() {
    let app = TestApp::new().await;

    let body = json!({
        "email": "rust@example.com",
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_returns_200_for_valid_post_request() {
    let app = TestApp::new().await;

    let body = json!({
        "email": "rust@example.com",
        "password": "password123"
    });

    let response = app.post_login(body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_returns_200_for_post_request() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_returns_200_for_valid_post_request() {
    let app = TestApp::new().await;

    let body = json!({
        "email": "rust@example.com",
        "loginAttemptId": "123456",
        "2FACode": "654321"
    });

    let response = app.post_verify_2fa(body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_returns_200_for_valid_post_request() {
    let app = TestApp::new().await;

    let body = json!({
        "token": "tokn1234567890..."
    });

    let response = app.post_verify_token(body).await;

    assert_eq!(response.status().as_u16(), 200);
}

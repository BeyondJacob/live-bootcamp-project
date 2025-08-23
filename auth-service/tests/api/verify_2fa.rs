use crate::helpers::TestApp;
use serde_json::json;

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
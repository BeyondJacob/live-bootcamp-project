use crate::helpers::TestApp;
use serde_json::json;

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
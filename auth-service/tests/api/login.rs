use crate::helpers::TestApp;
use serde_json::json;

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
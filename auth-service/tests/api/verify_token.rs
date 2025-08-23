use crate::helpers::TestApp;
use serde_json::json;

#[tokio::test]
async fn verify_token_returns_200_for_valid_post_request() {
    let app = TestApp::new().await;

    let body = json!({
        "token": "tokn1234567890..."
    });

    let response = app.post_verify_token(body).await;

    assert_eq!(response.status().as_u16(), 200);
}
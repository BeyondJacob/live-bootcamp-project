use crate::helpers::TestApp;

#[tokio::test]
async fn logout_returns_200_for_post_request() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}
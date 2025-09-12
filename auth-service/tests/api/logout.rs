use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;
use serde_json;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);
    
    let json: ErrorResponse = response
        .json()
        .await
        .expect("Could not deserialize response body to ErrorResponse");

    assert_eq!(json.error, "Missing auth token");
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401);
    
    let json: ErrorResponse = response
        .json()
        .await
        .expect("Could not deserialize response body to ErrorResponse");

    assert_eq!(json.error, "Invalid auth token");
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    // First, signup and login a user to get a valid JWT cookie
    let email = get_random_email();
    let password = "password123";
    
    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    });
    
    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);
    
    let login_body = serde_json::json!({
        "email": email,
        "password": password
    });
    
    let login_response = app.post_login(&login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);
    
    // Now test logout with the valid JWT cookie
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    // First, signup and login a user to get a valid JWT cookie
    let email = get_random_email();
    let password = "password123";
    
    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    });
    
    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);
    
    let login_body = serde_json::json!({
        "email": email,
        "password": password
    });
    
    let login_response = app.post_login(&login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);
    
    // First logout should succeed
    let first_logout_response = app.post_logout().await;
    assert_eq!(first_logout_response.status().as_u16(), 200);
    
    // Second logout should fail with 400 (missing cookie)
    let second_logout_response = app.post_logout().await;
    assert_eq!(second_logout_response.status().as_u16(), 400);
    
    let json: ErrorResponse = second_logout_response
        .json()
        .await
        .expect("Could not deserialize response body to ErrorResponse");

    assert_eq!(json.error, "Missing auth token");
}
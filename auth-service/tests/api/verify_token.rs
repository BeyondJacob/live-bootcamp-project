use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::cookie::CookieStore;

use crate::helpers::{get_random_email, TestApp};
use serde_json::json;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    // Send a malformed JSON body (missing the "token" field)
    let body = json!({
        "invalid_field": "some_value"
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 422);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let mut app = TestApp::new().await;

    // First, signup and login a user to get a valid JWT token
    let email = get_random_email();
    let password = "password123";

    let signup_body = json!({
        "email": email,
        "password": password,
        "requires2FA": false
    });

    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    let login_body = json!({
        "email": email,
        "password": password
    });

    let login_response = app.post_login(&login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);

    // Extract the JWT token from the cookie
    let url = reqwest::Url::parse(&app.address).unwrap();
    let cookies = app.cookie_jar.cookies(&url).expect("Should have cookies");
    let cookie_str = cookies.to_str().expect("Cookie should be valid string");

    let token = cookie_str
        .split(';')
        .find(|cookie| cookie.trim().starts_with(&format!("{}=", JWT_COOKIE_NAME)))
        .and_then(|cookie| cookie.split('=').nth(1))
        .expect("JWT cookie should be present")
        .to_string();

    // Now test verify-token with the valid JWT
    let body = json!({
        "token": token
    });

    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 200);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    // Send an invalid token
    let body = json!({
        "token": "invalid_token_12345"
    });

    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 401);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let mut app = TestApp::new().await;

    // First, signup and login a user to get a valid JWT token
    let email = get_random_email();
    let password = "password123";

    let signup_body = json!({
        "email": email,
        "password": password,
        "requires2FA": false
    });

    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    let login_body = json!({
        "email": email,
        "password": password
    });

    let login_response = app.post_login(&login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);

    // Extract the JWT token from the cookie
    let url = reqwest::Url::parse(&app.address).unwrap();
    let cookies = app.cookie_jar.cookies(&url).expect("Should have cookies");
    let cookie_str = cookies.to_str().expect("Cookie should be valid string");

    let token = cookie_str
        .split(';')
        .find(|cookie| cookie.trim().starts_with(&format!("{}=", JWT_COOKIE_NAME)))
        .and_then(|cookie| cookie.split('=').nth(1))
        .expect("JWT cookie should be present")
        .to_string();

    // Add token to banned store
    let mut banned_store = app.banned_token_store.write().await;
    banned_store.add_token(token.clone()).await.unwrap();
    drop(banned_store);

    // Now test verify-token with the banned JWT
    let body = json!({
        "token": token
    });

    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 401);

    app.clean_up().await;
}

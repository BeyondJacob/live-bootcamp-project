use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let mut app = TestApp::new().await;

    // Test with malformed JSON (missing password field)
    let test_cases = vec![
        json!({
            "email": "test@example.com"
            // Missing password field
        }),
        json!({
            "password": "password123"
            // Missing email field
        }),
        json!({
            // Empty JSON object
        }),
        json!("not an object"), // String instead of object
        json!(123),             // Number instead of object
    ];

    for invalid_body in test_cases {
        let response = app.post_login(&invalid_body).await;
        assert_eq!(response.status().as_u16(), 422);
    }

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;

    let test_cases = vec![
        json!({
            "email": "not-an-email",  // Invalid email format
            "password": "password123"
        }),
        json!({
            "email": "test@example.com",
            "password": "short"  // Password too short (less than 8 chars)
        }),
        json!({
            "email": "",  // Empty email
            "password": "password123"
        }),
        json!({
            "email": "test@example.com",
            "password": ""  // Empty password
        }),
    ];

    for invalid_body in test_cases {
        let response = app.post_login(&invalid_body).await;
        assert_eq!(response.status().as_u16(), 400);

        let error_response = response
            .json::<ErrorResponse>()
            .await
            .expect("Failed to deserialize error response");
        assert_eq!(error_response.error, "Invalid credentials");
    }

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let mut app = TestApp::new().await;

    // First, create a user to test against
    let valid_email = get_random_email();
    let valid_password = "password123";

    let signup_body = serde_json::json!({
        "email": &valid_email,
        "password": valid_password,
        "requires2FA": false
    });

    // Create the user
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Test with incorrect credentials
    let test_cases = vec![
        // Wrong password
        json!({
            "email": &valid_email,
            "password": "wrong_password"
        }),
        // Non-existent user
        json!({
            "email": "non_existent@example.com",
            "password": "password123"
        }),
    ];

    for invalid_body in test_cases {
        let response = app.post_login(&invalid_body).await;
        assert_eq!(response.status().as_u16(), 401);

        let error_response = response
            .json::<ErrorResponse>()
            .await
            .expect("Failed to deserialize error response");
        assert_eq!(error_response.error, "Incorrect credentials");
    }

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    // TODO: assert that `json_body.login_attempt_id` is stored inside `app.two_fa_code_store`

    app.clean_up().await;
}

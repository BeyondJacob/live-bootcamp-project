use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;
use serde_json::json;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

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
        json!(123), // Number instead of object
    ];

    for invalid_body in test_cases {
        let response = app.post_login(&invalid_body).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

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
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

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
}
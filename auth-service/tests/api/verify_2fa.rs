use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};
use serde_json::json;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let test_cases = vec![
        json!({
            "email": "test@example.com",
            "loginAttemptId": "123456"
            // Missing 2FACode
        }),
        json!({
            "email": "test@example.com",
            "2FACode": "654321"
            // Missing loginAttemptId
        }),
        json!({
            "loginAttemptId": "123456",
            "2FACode": "654321"
            // Missing email
        }),
        json!({
            // Empty object
        }),
        json!("not an object"), // String instead of object
    ];

    for malformed_body in test_cases {
        let response = app.post_verify_2fa(&malformed_body).await;
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
            "loginAttemptId": "123456",
            "2FACode": "654321"
        }),
        json!({
            "email": "test@example.com",
            "loginAttemptId": "not-a-uuid",  // Invalid UUID
            "2FACode": "654321"
        }),
        json!({
            "email": "test@example.com",
            "loginAttemptId": "123e4567-e89b-12d3-a456-426614174000",
            "2FACode": "12345"  // Invalid 2FA code (not 6 digits)
        }),
        json!({
            "email": "test@example.com",
            "loginAttemptId": "123e4567-e89b-12d3-a456-426614174000",
            "2FACode": "abcdef"  // Invalid 2FA code (not numeric)
        }),
    ];

    for invalid_body in test_cases {
        let response = app.post_verify_2fa(&invalid_body).await;
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

    // Create a test user with 2FA enabled
    let random_email = get_random_email();
    let signup_body = json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true
    });

    app.post_signup(&signup_body).await;

    // Login to get a valid login attempt ID and 2FA code
    let login_body = json!({
        "email": &random_email,
        "password": "password123"
    });

    let login_response = app.post_login(&login_body).await;
    let auth_response = login_response
        .json::<TwoFactorAuthResponse>()
        .await
        .unwrap();

    // Test with wrong 2FA code
    let verify_body = json!({
        "email": &random_email,
        "loginAttemptId": &auth_response.login_attempt_id,
        "2FACode": "123456"  // Wrong code but valid format
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);

    let error_response = response
        .json::<ErrorResponse>()
        .await
        .expect("Failed to deserialize error response");
    assert_eq!(error_response.error, "Incorrect credentials");

    // Test with wrong login attempt ID
    let verify_body = json!({
        "email": &random_email,
        "loginAttemptId": "123e4567-e89b-12d3-a456-426614174000",  // Wrong ID
        "2FACode": "123456"
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);

    // Test with non-existent email
    let verify_body = json!({
        "email": "nonexistent@example.com",
        "loginAttemptId": &auth_response.login_attempt_id,
        "2FACode": "123456"
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let mut app = TestApp::new().await;

    // Create a test user with 2FA enabled
    let random_email = get_random_email();
    let signup_body = json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true
    });

    app.post_signup(&signup_body).await;

    let login_body = json!({
        "email": &random_email,
        "password": "password123"
    });

    // First login
    let first_login_response = app.post_login(&login_body).await;
    let first_auth_response = first_login_response
        .json::<TwoFactorAuthResponse>()
        .await
        .unwrap();

    // Get the 2FA code from the store
    let two_fa_code_store = app.two_fa_code_store.read().await;
    let email = Email::parse(random_email.clone()).unwrap();
    let (_, first_code) = two_fa_code_store.get_code(&email).await.unwrap();
    drop(two_fa_code_store); // Release the lock

    // Second login (this should overwrite the first 2FA code)
    let second_login_response = app.post_login(&login_body).await;
    let second_auth_response = second_login_response
        .json::<TwoFactorAuthResponse>()
        .await
        .unwrap();

    // Try to verify with the first login's 2FA code
    let verify_body = json!({
        "email": &random_email,
        "loginAttemptId": &first_auth_response.login_attempt_id,
        "2FACode": first_code.as_ref()
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);

    let error_response = response
        .json::<ErrorResponse>()
        .await
        .expect("Failed to deserialize error response");
    assert_eq!(error_response.error, "Incorrect credentials");

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let mut app = TestApp::new().await;

    // Create a test user with 2FA enabled
    let random_email = get_random_email();
    let signup_body = json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true
    });

    app.post_signup(&signup_body).await;

    // Login to get a valid login attempt ID
    let login_body = json!({
        "email": &random_email,
        "password": "password123"
    });

    let login_response = app.post_login(&login_body).await;
    let auth_response = login_response
        .json::<TwoFactorAuthResponse>()
        .await
        .unwrap();

    // Get the 2FA code from the store
    let two_fa_code_store = app.two_fa_code_store.read().await;
    let email = Email::parse(random_email.clone()).unwrap();
    let (_, two_fa_code) = two_fa_code_store.get_code(&email).await.unwrap();
    drop(two_fa_code_store); // Release the lock

    // Verify with the correct 2FA code
    let verify_body = json!({
        "email": &random_email,
        "loginAttemptId": &auth_response.login_attempt_id,
        "2FACode": two_fa_code.as_ref()
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Assert the auth cookie is set
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let mut app = TestApp::new().await;

    // Create a test user with 2FA enabled
    let random_email = get_random_email();
    let signup_body = json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true
    });

    app.post_signup(&signup_body).await;

    // Login to get a valid login attempt ID
    let login_body = json!({
        "email": &random_email,
        "password": "password123"
    });

    let login_response = app.post_login(&login_body).await;
    let auth_response = login_response
        .json::<TwoFactorAuthResponse>()
        .await
        .unwrap();

    // Get the 2FA code from the store
    let two_fa_code_store = app.two_fa_code_store.read().await;
    let email = Email::parse(random_email.clone()).unwrap();
    let (_, two_fa_code) = two_fa_code_store.get_code(&email).await.unwrap();
    drop(two_fa_code_store); // Release the lock

    // Verify with the correct 2FA code (first time)
    let verify_body = json!({
        "email": &random_email,
        "loginAttemptId": &auth_response.login_attempt_id,
        "2FACode": two_fa_code.as_ref()
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Try to verify with the same 2FA code again (should fail)
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);

    let error_response = response
        .json::<ErrorResponse>()
        .await
        .expect("Failed to deserialize error response");
    assert_eq!(error_response.error, "Incorrect credentials");

    app.clean_up().await;
}

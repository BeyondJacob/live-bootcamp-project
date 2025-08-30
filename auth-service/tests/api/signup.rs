use auth_service::routes::SignupResponse;
use crate::helpers::{get_random_email, TestApp};
use serde_json::json;

#[tokio::test]
async fn signup_returns_200_for_valid_post_request() {
    let app = TestApp::new().await;

    let body = json!({
        "email": "rust@example.com",
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&body).await;

    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let body = json!({
        "email": "rust@example.com",
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&body).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // TODO: add more malformed input test cases
    let test_cases = [
        // missing email
        json!({
            "password": "password123",
            "requires2FA": true
        }),
        // missing password
        json!({
            "email": random_email,
            "requires2FA": true
        }),
        // missing requires2FA
        json!({
            "email": random_email,
            "password": "password123"
        }),
        // wrong type for requires2FA
        json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": "nope"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

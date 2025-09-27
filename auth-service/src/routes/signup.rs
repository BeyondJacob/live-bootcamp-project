use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserStoreError},
};

#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Parse email and password
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    // Add user to store
    if let Err(e) = user_store.add_user(user).await {
        return Err(AuthAPIError::UnexpectedError(e.into()));
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
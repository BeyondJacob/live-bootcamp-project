use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, UserStoreError}
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
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

    let user_store = &state.user_store.read().await;

    // Validate user credentials
    if let Err(e) = user_store.validate_user(&email, &password).await {
        match e {
            UserStoreError::UserNotFound => return Err(AuthAPIError::IncorrectCredentials),
            UserStoreError::InvalidCredentials => return Err(AuthAPIError::IncorrectCredentials),
            _ => return Err(AuthAPIError::UnexpectedError),
        }
    }

    // Get user details (though we don't use them yet)
    let _user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };
    
    Ok(StatusCode::OK.into_response())
}
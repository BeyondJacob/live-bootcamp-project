use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie;

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::constants::JWT_COOKIE_NAME,
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let Some(cookie) = jar.get(JWT_COOKIE_NAME) else {
        return (jar, Err(AuthAPIError::MissingToken));
    };

    let token = cookie.value().to_owned();

    // Validate JWT token by calling `validate_token` from the auth service.
    // If the token is valid you can ignore the returned claims for now.
    // Return AuthAPIError::InvalidToken is validation fails.
    // First validate the token normally
    use crate::utils::auth::validate_token;
    if let Err(_) = validate_token(&token).await {
        return (jar, Err(AuthAPIError::InvalidToken));
    }
    
    // Then check if it's banned
    let banned_store = state.banned_token_store.read().await;
    if let Ok(true) = banned_store.is_banned(&token).await {
        return (jar, Err(AuthAPIError::InvalidToken));
    }
    drop(banned_store);

    // Add token to banned token store
    let mut banned_store = state.banned_token_store.write().await;
    if let Err(_) = banned_store.add_token(token).await {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    // Remove JWT cookie from the CookieJar
    let jar = jar.remove(cookie::Cookie::build(JWT_COOKIE_NAME).build());

    (jar, Ok(StatusCode::OK))
}
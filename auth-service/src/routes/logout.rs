use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie;
use axum_extra::extract::CookieJar;
use secrecy::Secret;

use crate::{app_state::AppState, domain::AuthAPIError, utils::constants::JWT_COOKIE_NAME};

#[tracing::instrument(name = "Logout", skip_all)]
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

    // Validate JWT token and check if it's banned
    use crate::utils::auth::validate_token;
    if let Err(_) = validate_token(&token, state.banned_token_store.clone()).await {
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    // Add token to banned token store
    let mut banned_store = state.banned_token_store.write().await;
    if let Err(e) = banned_store.add_token(Secret::new(token)).await {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    // Remove JWT cookie from the CookieJar
    let jar = jar.remove(cookie::Cookie::build(JWT_COOKIE_NAME).build());

    (jar, Ok(StatusCode::OK))
}

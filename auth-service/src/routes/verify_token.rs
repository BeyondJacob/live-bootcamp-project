use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, utils::auth::validate_token};

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}

pub async fn verify_token(
    State(state): State<AppState>,
    Json(body): Json<VerifyTokenRequest>,
) -> impl IntoResponse {
    // First validate the token
    if let Err(_) = validate_token(&body.token).await {
        return StatusCode::UNAUTHORIZED;
    }
    
    // Then check if it's banned
    let banned_store = state.banned_token_store.read().await;
    match banned_store.is_banned(&body.token).await {
        Ok(true) => StatusCode::UNAUTHORIZED,
        Ok(false) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}
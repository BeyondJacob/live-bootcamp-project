use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, utils::auth::validate_token};

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}

#[tracing::instrument(name = "Verify token", skip_all)]
pub async fn verify_token(
    State(state): State<AppState>,
    Json(body): Json<VerifyTokenRequest>,
) -> impl IntoResponse {
    // Validate the token and check if it's banned
    match validate_token(&body.token, state.banned_token_store.clone()).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}

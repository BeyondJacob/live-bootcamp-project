use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::utils::auth::validate_token;

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}

pub async fn verify_token(
    request: Result<Json<VerifyTokenRequest>, JsonRejection>,
) -> Result<impl IntoResponse, StatusCode> {
    let Json(body) = request.map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;
    
    // Validate the token
    match validate_token(&body.token).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
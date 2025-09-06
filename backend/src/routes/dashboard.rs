use axum::{
    extract::{State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::utils::auth::verify_token;
use serde_json::json;

/// Dashboard handler with manual header extraction
pub async fn dashboard(
    State(_pool): State<PgPool>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let token = match headers.get("authorization") {
        Some(value) => value.to_str().unwrap_or(""),
        None => return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Missing Authorization header" }))).into_response(),
    };

    let token = token.strip_prefix("Bearer ").unwrap_or("");

    match verify_token(token) {
        Ok(data) => {
            let claims = data.claims;

            let response = json!({
                "message": "Welcome to the dashboard!",
                "email": claims.sub,
                "role": claims.role,
                "info": {
                    "visitor_count": 1234,
                    "inquiry_count": 56
                }
            });

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(_) => {
            (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Invalid or expired token" }))).into_response()
        }
    }
}

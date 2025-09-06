use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use bcrypt::{hash, verify};
use serde_json::json;

use crate::models::users::{User, NewUser};
use crate::models::auth::LoginPayload;
use crate::utils::auth::generate_token;

/// Signup
pub async fn signup(State(pool): State<PgPool>, Json(payload): Json<NewUser>) -> impl IntoResponse {
    // hash password
    let hashed = match hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"Hashing failed"}))).into_response()
    };

    let res = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password, organisation_name, is_admin, created_at)
        VALUES ($1, $2, $3, $4, false, NOW())
        RETURNING id, username, email, password, organisation_name, is_admin, created_at"
    )
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(&hashed)
        .bind(&payload.organisation_name)
        .fetch_one(&pool)
        .await;

    match res {
        Ok(user) => {
            let token = generate_token(&user.email, if user.is_admin { "admin" } else { "user" }).unwrap();
            (StatusCode::CREATED, Json(json!({ "user": user, "token": token }))).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"DB error"}))).into_response()
    }
}

/// Login  ok
pub async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    let rec = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await;

    match rec {
        Ok(Some(user)) => {
            if verify(&payload.password, &user.password).unwrap_or(false) {
                let token = generate_token(&user.email, if user.is_admin { "admin" } else { "user" }).unwrap();
                (StatusCode::OK, Json(json!({ "user": user, "token": token }))).into_response()
            } else {
                (StatusCode::UNAUTHORIZED, Json(json!({"error":"Invalid credentials"}))).into_response()
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, Json(json!({"error":"Invalid credentials"}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"DB error"}))).into_response()
    }
}

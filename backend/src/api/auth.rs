use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use bcrypt::{hash, verify};
use serde_json::json;
use crate::models::users::{User, NewUser,SafeUser};
use crate::models::auth::LoginPayload;
use crate::utils::auth::generate_token;

pub async fn signup(State(pool): State<PgPool>, Json(payload): Json<NewUser>) -> impl IntoResponse {
    // hash password
    let hashed = match hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"Hashing failed"}))).into_response(),
    };

    // insert user
    let res = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password, organisation_name, user_type, created_at)
         VALUES ($1, $2, $3, $4, $5, NOW())
         RETURNING id, username, email, password, organisation_name, user_type, created_at"
    )
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(&hashed)
        .bind(&payload.organisation_name)
        .bind(&payload.user_type)
        .fetch_one(&pool)
        .await;

    match res {
        Ok(user) => {
            let token = match generate_token(&user.email, &user.user_type) {
                Ok(t) => t,
                Err(err) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("JWT error: {}", err)}))).into_response()
                }
            };

            (StatusCode::CREATED, Json(json!({
                "user": SafeUser::from(user),
                "token": token
            }))).into_response()
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("DB error: {}", err)}))).into_response(),
    }
}

/// Login
pub async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    let rec = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await;

    match rec {
        Ok(Some(user)) => {
            if verify(&payload.password, &user.password).unwrap_or(false) {
                let token = match generate_token(&user.email, &user.user_type) {
                    Ok(t) => t,
                    Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("JWT error: {}", err)}))).into_response(),
                };

                (StatusCode::OK, Json(json!({ "user": SafeUser::from(user), "token": token }))).into_response()
            } else {
                (StatusCode::UNAUTHORIZED, Json(json!({"error":"Invalid credentials"}))).into_response()
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, Json(json!({"error":"Invalid credentials"}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("DB error: {}", err)}))).into_response(),
    }
}

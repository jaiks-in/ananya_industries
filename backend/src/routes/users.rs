use crate::models::users::{User,NewUser};
use axum::{Json,response::IntoResponse,extract::{State,Path},http::StatusCode,};
use sqlx::PgPool;


pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<NewUser>,
) -> impl IntoResponse {
    let res = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password, is_admin, created_at)
         VALUES ($1, $2, $3, false, NOW())
         RETURNING *"
    )
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(&payload.password)
        .fetch_one(&pool)
        .await;

    match res {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => {
            eprintln!("DB insert error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR,
             Json(serde_json::json!({ "error": "DB error" }))
            ).into_response()
        }
    }
}

pub async fn get_user_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
        .bind(id)   // yahan normal `id` chalega
        .fetch_one(&pool)
        .await;


    match user {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => {
            eprintln!("DB insert error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR,
             Json(serde_json::json!({ "error": "DB error" }))
            ).into_response()
        }
    }
}

use axum::{response::IntoResponse, Json};
use axum::extract::State;
use chrono::{ Utc};
use sqlx::PgPool;
use crate::models::contacts::{Contact, CreateContact};
pub async fn get_all_contact_data_handler(State(pool):State<PgPool>) -> impl IntoResponse {
        let get_all_contact=sqlx::query_as::<_, Contact>("SELECT * FROM contacts").fetch_all(&pool);
    match get_all_contact.await {
        Ok(data)=>(axum::http::StatusCode::OK,Json(data)).into_response(),
        Err(error)=>{
            println!("{}", error);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to create contact" })),
            )
                .into_response()
        }
    }
}pub async fn create_contact_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateContact>,
) -> impl IntoResponse {
    let create_contact_result = sqlx::query_as::<_, Contact>(
        r#"
    INSERT INTO contacts(name, email, mobile, message, created_at)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING id, name, email, mobile, message, created_at
    "#
    )
        .bind(payload.name)
        .bind(payload.email)
        .bind(payload.mobile)
        .bind(payload.message)
        .bind(Utc::now().naive_utc())
        .fetch_one(&pool)
        .await;


    match create_contact_result {
        Ok(contact) => (axum::http::StatusCode::CREATED, Json(contact)).into_response(),
        Err(err) => {
            eprintln!("Failed to create contact: {:?}", err);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to create contact" })),
            )
                .into_response()
        }
    }
}

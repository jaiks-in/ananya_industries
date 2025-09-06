use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

/// Full user model
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,          // will be sent in responses if you want
    pub organisation_name:Option<String>,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
}

/// Payload for creating a new user
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub organisation_name: String,
}

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
    pub user_type:String,
    pub created_at: NaiveDateTime,
}

/// Payload for creating a new user
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub user_type:String,
    pub organisation_name: String,
}
#[derive(Serialize)]
pub struct SafeUser {
    id: i32,
    username: String,
    email: String,
    organisation_name: String,
    user_type: String,
    created_at: String,
}


impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            organisation_name: user.organisation_name.unwrap(),
            user_type: user.user_type,
            created_at: user.created_at.to_string(),
        }
    }
}
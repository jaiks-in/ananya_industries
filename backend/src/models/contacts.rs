use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug,Serialize,Deserialize)]
pub struct CreateContact{
    pub name :String,
    pub organization:String,
    pub email:String,
    pub mobile:String,
    pub message:String,
}
#[derive(Debug,Serialize,FromRow)]
pub struct Contact{
    pub id:i32,
    pub name:String,
    pub organization:String,
    pub email:String,
    pub mobile:String,
    pub message:String,
    pub created_at:NaiveDateTime,
}
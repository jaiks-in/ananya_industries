use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug,Serialize,Deserialize)]
pub struct CreateContact{
    pub name :String,
    pub email:String,
    pub mobile:String,
    pub message:String,
    pub organisation_name:String,
}
#[derive(Debug,Serialize,FromRow)]
pub struct Contact{
    pub id:i32,
    pub name:String,
    pub email:String,
    pub mobile:String,
    pub message:String,
    pub organisation_name:String,
    pub created_at:NaiveDateTime,

}
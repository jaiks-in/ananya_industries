use serde::{Serialize,Deserialine};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug,Deserialine)]
pub struct CreateContact{
    pub name :String,
    pub email:String,
    pub mobile:String,
    pub messsage:string,
}
#[derive(Debug,Serialize,FromRow)]
pub struct Contact{
    pub id:i32,
    pub name:String,
    pub email:String,
    pub mobile:String,
    pub message:String,
    pub created_at:NaiveDateTime,
}
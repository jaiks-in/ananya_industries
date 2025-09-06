use std::sync::OnceLock;

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug,Serialize)]
pub struct LoginResponse{
    pub token:String
}
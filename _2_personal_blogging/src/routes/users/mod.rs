use serde::{Deserialize, Serialize};

pub mod create_user;
pub mod fetch_users;

#[derive(Debug, Deserialize)]
pub struct RequestUser {
    pub username: String, 
    pub password: String,
    pub email: String
}

#[derive(Debug, Serialize)]
pub struct ResponseUser {
    id: i32,
    username: String, 
    email: String,
    token: Option<String>
}
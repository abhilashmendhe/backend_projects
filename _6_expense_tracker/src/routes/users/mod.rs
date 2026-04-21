use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod create_user;
pub mod delete_user;
pub mod get_user;
pub mod login;
pub mod logout;
pub mod update_password;
pub mod activate_acc;
pub mod user_midd_extractor;

#[derive(Debug, Deserialize)]
pub struct RequestUser {
    pub username: String, 
    pub password: String,
    pub email: String
}

#[derive(Debug, Serialize)]
pub struct ResponseUser {
    pub id: i32, 
    username: String, 
    email: String,
    created_at: DateTime<Utc>,
    token: Option<String>
}
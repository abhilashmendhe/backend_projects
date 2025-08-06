use serde::{Deserialize, Serialize};

pub mod create_user;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    data: ResponseUser
}

#[derive(Deserialize, Serialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    username: String,
    password: String
}
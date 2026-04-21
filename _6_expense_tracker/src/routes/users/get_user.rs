use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{models::users_model::UserModel, routes::users::ResponseUser, utils::{errors::{AppError, ExpenseTrackerErr}}};

pub async fn get_user(
    user: UserModel,
    req_path: web::Path<i32>
) -> Result<HttpResponse, ExpenseTrackerErr> {

    let id = req_path.into_inner();
    
    if id != user.id {
        return Err(ExpenseTrackerErr::AppError(AppError::new(StatusCode::NOT_FOUND, "User not found!")));
    }
    let resp_user = ResponseUser {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at.unwrap(),
        token: user.token,
    };
    Ok(HttpResponse::Ok().json(resp_user))
}
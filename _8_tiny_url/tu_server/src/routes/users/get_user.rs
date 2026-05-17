use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{
    models::user_model::UserModel,
    utils::{
        app_state::AppState,
        errors::{AppError, TinyUrlError},
    },
};

pub async fn get_user(
    id: web::Path<i32>,
    user: UserModel,
    _app_data: web::Data<AppState>,
) -> Result<HttpResponse, TinyUrlError> {
    let id = id.into_inner();

    if id != user.id as i32 {
        return Err(TinyUrlError::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "You are not the user for this ID",
        )));
    }
    // 1. check if already user delted
    if let Some(_) = user.deleted_at {
        return Err(TinyUrlError::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "User doesn't exists!",
        )));
    }
    Ok(HttpResponse::Ok().json(user))
}

use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{
    models::user_model::UserModel,
    utils::{
        app_state::AppState,
        errors::{AppError, TinyUrlError},
    },
};

pub async fn delete_user(
    id: web::Path<i32>,
    user: UserModel,
    app_data: web::Data<AppState>,
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

    // 2. delete user by setting deleted_at to time
    sqlx::query!(
        r#"
         UPDATE users SET deleted_at=NOW() WHERE id=$1
        "#,
        user.id as i32
    )
    .execute(app_data.pool())
    .await
    .map_err(|err| {
        tracing::error!("Error deleting user: {:?}", err);
        // println!("{:?}",);
        if let None = err.as_database_error() {
            return TinyUrlError::AppError(AppError::new(
                StatusCode::NOT_FOUND,
                "User not found!",
            ));
        }
        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL SERVER ERROR",
        ))
    })?;

    Ok(HttpResponse::Ok().body(format!("User with id: {} delete!", id)))
}

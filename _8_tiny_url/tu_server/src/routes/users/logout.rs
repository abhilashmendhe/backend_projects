use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{
    models::user_model::UserModel,
    utils::{
        app_state::AppState,
        errors::{AppError, TinyUrlError},
    },
};

pub async fn logout(
    user: UserModel,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, TinyUrlError> {
    // 1. update user row, set token to null
    sqlx::query!(
        r#"
            UPDATE users SET token=NULL WHERE id=$1
        "#,
        user.id as i32
    )
    .execute(app_data.pool())
    .await
    .map_err(|err| {
        tracing::error!("Failed to `update` (log out) users table: {:?}", err);
        if let None = err.as_database_error() {
            return TinyUrlError::AppError(AppError::new(StatusCode::NOT_FOUND, "User not found!"));
        }

        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update users table",
        ))
    })?;

    Ok(HttpResponse::Ok().body("Logged Out!"))
}

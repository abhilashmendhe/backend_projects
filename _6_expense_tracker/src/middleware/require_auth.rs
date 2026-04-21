use actix_web::{HttpMessage, body, dev::{ServiceRequest, ServiceResponse}, http::StatusCode, middleware::Next, web};

use crate::{models::users_model::UserModel, utils::{app_state::AppState, errors::{AppError, ExpenseTrackerErr}, jwt::validate_token}};

pub async fn require_auth(
    req: ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {

    let app_data = match req.app_data::<web::Data<AppState>>().cloned() {
        Some(app_data) => app_data,
        None => {
            return Err(ExpenseTrackerErr::AppError(
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            ).into());
        },
    };

    if let None = req.headers().get("authorization") {
        return Err(ExpenseTrackerErr::AppError(
            AppError::new(StatusCode::UNAUTHORIZED, "Not authenticated")
        ).into());
    }
    let auth_header_token = if let Some(value) = req.headers().get("authorization") {
        value.to_str()
            .map_err(|err| {
                tracing::error!("{:?}",err);
                return ExpenseTrackerErr::AppError(
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
                );
            })?
    }
    else {
        return Err(ExpenseTrackerErr::AppError(
            AppError::new(StatusCode::UNAUTHORIZED, "Not authenticated")
        ).into());
    };

    if !auth_header_token.starts_with("Bearer ") {
        return Err(ExpenseTrackerErr::AppError(
            AppError::new(StatusCode::UNAUTHORIZED, "Not authenticated")
        ).into());
    }

    let token = auth_header_token.trim_start_matches("Bearer ").trim();

    validate_token(app_data.config.secret(), token)?;

    println!("Done validation in JWT Auth. Now fetch user");
    let user = sqlx::query_as!(
        UserModel,
        r#"SELECT * FROM users WHERE token=$1"#,
        token
    ).fetch_one(&app_data.pool)
    .await
    .map_err(|err| {
        tracing::error!("Error fetching user: {:?}",err);
        if let None = err.as_database_error() {
            return ExpenseTrackerErr::AppError(AppError::new(StatusCode::NOT_FOUND, "User not found!"));
        }
        ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR"))
    })?;

    req.extensions_mut().insert(user);
    let res = next.call(req).await?;
    Ok(res)
}
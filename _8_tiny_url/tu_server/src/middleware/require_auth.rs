use actix_web::{
    HttpMessage, body,
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::Next,
    web,
};

use crate::{
    models::user_model::UserModel,
    utils::{
        app_state::AppState,
        errors::{AppError, TinyUrlError},
        jwt::validate_token,
    },
};

pub async fn require_auth(
    req: ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
    let app_data = match req.app_data::<web::Data<AppState>>().cloned() {
        Some(app_data) => app_data,
        None => {
            return Err(TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            ))
            .into());
        }
    };

    println!("HEADER VALUE: {:?}", req.headers().get("authorization"));
    let auth_header_token = if let Some(header_value) = req.headers().get("authorization") {
        header_value.to_str().map_err(|err| {
            tracing::error!("Failed to get header token: {:?}", err);
            return TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error reading token from header (bearer)",
            ));
        })?
    } else {
        return Err(TinyUrlError::AppError(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Not authorized!",
        ))
        .into());
    };

    if !auth_header_token.starts_with("Bearer ") {
        return Err(TinyUrlError::AppError(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Not authenticated",
        ))
        .into());
    }

    let token = auth_header_token.trim_start_matches("Bearer ").trim();
    // println!("Secret: {}", app_data.config.secret());
    // println!("Token: {}", token);
    validate_token(&app_data.config().jwt_secret(), token)?;

    tracing::info!("Done validation in JWT Auth. Now fetch user");
    let user = sqlx::query_as!(UserModel, r#"SELECT * FROM users WHERE token=$1"#, token)
        .fetch_one(app_data.pool())
        .await
        .map_err(|err| {
            tracing::error!("Error fetching user: {:?}", err);
            if let None = err.as_database_error() {
                return TinyUrlError::AppError(AppError::new(
                    StatusCode::NOT_FOUND,
                    "User not found or already logged out!",
                ));
            }
            TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL SERVER ERROR",
            ))
        })?;

    req.extensions_mut().insert(user);

    let res = next.call(req).await?;
    Ok(res)
}

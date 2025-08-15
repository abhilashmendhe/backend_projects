use axum::{body::Body, extract::{Request, State}, http::{HeaderMap, StatusCode}, middleware::Next, response::Response};
use sqlx::PgPool;
use sqlx::Row;

use crate::{models::user::UserDB, utils::{config::Config, errors::AppError, jwt::validate_token}};

pub async fn require_authentication(
    State(db): State<PgPool>,
    State(config): State<Config>,
    headers: HeaderMap,
    mut request: Request<Body>,
    next: Next
) -> Result<Response, AppError> {

    // 1. extract header token
    let header_token = if let Some(token) = headers.get("x-auth-token") {
        token.to_str()
            .map_err(|err|{
                eprintln!("Error extracting token: {:?}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
            })
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "Not authenticated"));
    }?;

    // 2. validate if token w.r.t jwt_secret
    validate_token(&config.jwt_secret(), header_token).await?;

    // 3. find user for the matched token
    let user_row = sqlx::query(r#"
        SELECT * FROM users WHERE token=$1
    "#)
    .bind(header_token)
    .fetch_one(&db)
    .await
    .map_err(|error| {
        eprintln!("Error getting user by token: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was a problem getting your account",
        )
    })?;
    let user = UserDB {
        id: user_row.get("id"),
        username: user_row.get("username"),
        password: user_row.get("password"),
        created_at: user_row.get("created_at"),
        deleted_at: user_row.get("deleted_at"),
        token: user_row.get("token"),
        email: user_row.get("email"),
    };
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
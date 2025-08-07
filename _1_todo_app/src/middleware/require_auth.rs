use axum::{body::Body, extract::State, http::{HeaderMap, Request, StatusCode}, middleware::Next, response::Response};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::users::{self, Entity as Users};

use crate::utils::{app_error::AppError, app_state::AppState, jwt::validate_token};

pub async fn require_auth(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request<Body>,
    next: Next 
) -> Result<Response, AppError> {
    
    let header_token = if let Some(token) = headers.get("x-auth-token") {
        
        token.to_str()
            .map_err(|err| {
                eprintln!("Error extracting token from headers: {:?}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    "Error reading token"
                )
            })?
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED, 
            "Missing Authentication token"
        ))
    };
    validate_token(&state.jwt_secret, header_token)?;
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(header_token)))
        .one(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user by token: {:?}",err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
            "There was a problem getting your account"
            )
        })?;
    if let Some(user) = user {
         // if logged in
         request.extensions_mut().insert(user);
    } else {
        // if not logged in
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED, 
            "You are not authorized for this"
        ));
    }
    Ok(next.run(request).await)
}
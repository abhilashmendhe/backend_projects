
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, TryIntoModel};

use crate::database::users;
use crate::routes::users::{RequestCreateUser, ResponseDataUser, ResponseUser};
use crate::utils::app_error::AppError;
use crate::utils::app_state::AppState;
use crate::utils::jwt::create_token;

pub async fn create_user(
    State(state): State<AppState>,
    Json(request_user): Json<RequestCreateUser>
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel{ ..Default::default() };
    new_user.username = Set(request_user.username);
    new_user.password = Set(request_user.password);
    let token = create_token(&state.jwt_secret)?;
    new_user.token = Set(Some(token));
    let db = state.db;
    let user= new_user
        .save(&db)
        .await
        .map_err(|error| {
            // dbg!(error);
            eprintln!("Error creating user: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Something went wrong. Please try again"
            )
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting user back into model : {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Error creating user")
        })?;

    Ok(Json(
        ResponseDataUser { data: ResponseUser { 
            id: user.id,
            username: user.username,
            token: user.token.unwrap()
        } }
    ))
}
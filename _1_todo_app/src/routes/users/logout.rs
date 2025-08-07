use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, IntoActiveModel};

use crate::{database::users, utils::{app_error::AppError, app_state::AppState}};
/*
    we need middleware to extract the token, 
    and then get the user....
*/
pub async fn logout(
    State(state): State<AppState>, 
    Extension(user): Extension<users::Model>,
    
) -> Result<StatusCode, AppError> {
    println!("Called logout");
    let mut user = user.into_active_model();
    user.token = Set(None);
    user
        .save(&state.db)
        .await
        .map_err(|err| {
            eprint!("{:?}",err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was problem logging out..."
            )
        })?;
    Ok(StatusCode::OK)
}
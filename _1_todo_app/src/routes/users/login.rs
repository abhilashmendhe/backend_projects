use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use crate::database::users::{self, Entity as Users};
use crate::routes::users::ResponseUser;
use crate::utils::jwt::{create_token, verify_jwt};
use crate::{routes::users::{RequestCreateUser, ResponseDataUser}, utils::{app_error::AppError, app_state::AppState}};

pub async fn login(
    State(state): State<AppState>, 
    Json(request_user): Json<RequestCreateUser>
) -> Result<Json<ResponseDataUser>, AppError> {

    let user = Users::find()
            .filter(users::Column::Username.eq(request_user.username))
            .one(&state.db)
            .await
            .map_err(|err| {
                eprintln!("Error getting user for login: {:?}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error logging in, please try again later"
                )
            })?;

    if let Some(user) = user {

        let token = create_token(&state.jwt_secret, user.username.clone())?;
        let mut user_active_model = user.clone().into_active_model();
        user_active_model.token = Set(Some(token.clone()));

        // now save again after creating a token
        user_active_model
            .save(&state.db)
            .await
            .map_err(|err| {
                eprintln!("Error adding token to user in db: {:?}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error logging you in..."
                )
            })?;
        if verify_jwt(&request_user.password, &user.password)? {
            Ok(Json(
                ResponseDataUser { data: ResponseUser { 
                    id: user.id,
                    username: user.username,
                    token: token
                } }
            ))
        } else {
            Err(
                AppError::new(
                    StatusCode::NOT_FOUND, 
                    "Bad username or password")
            )
        }
    } else {
        Err(
            AppError::new(StatusCode::NOT_FOUND, "Bad username or passord")
        )
    }
}
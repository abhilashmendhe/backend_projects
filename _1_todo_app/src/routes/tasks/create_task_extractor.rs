use axum::{extract::{FromRequest, Request}, http::StatusCode, Json, RequestExt};
use serde::Deserialize;
use validator::Validate;

use crate::utils::app_error::AppError;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, max = 1))]
    pub priority: Option<String>,
    #[validate(required(message = "missing task title"))]
    pub title: Option<String>,
    pub description: Option<String>,
}

impl<S> FromRequest<S> for ValidateCreateTask 
where
    S: Send + Sync + Clone
{
    type Rejection = AppError;
    async  fn from_request(
        req: Request, 
        _state: &S) -> Result<Self, Self::Rejection> {
            // println!("In create task extractor");
        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|error| {
                eprintln!("Error extracting new task: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again",
                )
            })?;

        if let Err(errors) = task.validate() {
            let field_errors = errors.field_errors();
            for (_, error) in field_errors {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    error.first().unwrap().clone().message.unwrap().to_string(), // feel safe unwrapping because we know there is at least one error, and we only care about the first for this api
                ));
            }
        }

        Ok(task)
    }
}
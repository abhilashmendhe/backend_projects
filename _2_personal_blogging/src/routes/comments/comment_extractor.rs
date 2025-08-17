use axum::{extract::FromRequest, http::StatusCode, Json, RequestExt};
use serde::Deserialize;
use validator::Validate;

use crate::utils::errors::AppError;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCommentPost {    
    #[validate(required(message = "missing comment"), length(min=8))]
    pub comment: Option<String>,
    pub parent_comment_id: Option<i32>
}

impl<S> FromRequest<S> for ValidateCommentPost 
where 
    S: Send + Sync + Clone {

        type Rejection = AppError;
        async  fn from_request(req:axum::extract::Request, _state: &S,) -> Result<Self,Self::Rejection> {
            let Json(comment) = req.extract::<Json<ValidateCommentPost>,_>()
            .await
            .map_err(|err|{
                eprintln!("Error extracting comment details: {:?}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    "Something went wrong. Please try again!"
                )
            })?;

            if let Err(errors) = comment.validate() {
                let field_errs = errors.field_errors();
                for (_, error) in field_errs {
                    let first_message = if let Some(validate_error) = error.first() {
                        // println!("{:?}", validate_error);
                        if let Some(msg) = &validate_error.message {
                            msg.to_string()
                        } else {
                            "Comment should not be empty".to_string()
                        }
                    } else {
                        "Unknown err....".to_string()
                    };
                    return Err(AppError::new(
                        StatusCode::BAD_REQUEST,
                        first_message, 
                    ));
                }
            }

            Ok(comment)
        }
    }
use axum::{extract::FromRequest, http::StatusCode, Json, RequestExt};
use serde::Deserialize;
use validator::Validate;

use crate::utils::errors::AppError;


#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCreatePost {
    #[validate(required(message = "missing blog post title"), length(min = 12, max = 64))]
    pub title: Option<String>,
    #[validate(required(message = "missing blog post content"), length(min = 8))]
    pub content: Option<String>,
    #[validate(required(message = "missing published"))]
    pub published: Option<bool>,
    pub login_required: Option<bool>
}

impl<S> FromRequest<S> for ValidateCreatePost
where
    S: Send + Sync + Clone {

        type Rejection = AppError;
    async fn from_request(
        req:axum::extract::Request,
        _state: &S
    ) -> Result<Self,Self::Rejection> {
        
        // println!("In blog post validate");
        
        let Json(post) = req.extract::<Json<ValidateCreatePost>,_>()
        .await
        .map_err(|err|{
            eprintln!("Error extracting blog post details: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Something went wrong. Please try again!"
            )
        })?;
        
        if let Err(errors) = post.validate() {
            let field_errs = errors.field_errors();
            for (_, error) in field_errs {
                let first_message = if let Some(validate_error) = error.first() {
                    // println!("{:?}", validate_error);
                    if let Some(msg) = &validate_error.message {
                        msg.to_string()
                    } else {
                        "Title, content and published should not be empty".to_string()
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
        Ok(post)
    }
}
use crate::utils::app_error::AppError;

pub async fn hello_world() -> Result<String, AppError> {
    Ok("hello world".into())
}
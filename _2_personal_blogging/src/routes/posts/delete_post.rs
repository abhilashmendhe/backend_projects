use axum::{extract::{Path, State}, http::StatusCode, Extension};
use chrono::Utc;
use sqlx::PgPool;

use crate::{models::user_model::UserDB, utils::errors::AppError};

pub async fn delete_post(
    Extension(user): Extension<UserDB>,
    State(db): State<PgPool>,
    Path(post_id): Path<i32>
) -> Result<(StatusCode, String), AppError> {
    println!("In delete post");

    let now = Utc::now();
    
    let pq_res = sqlx::query(r#"
        UPDATE posts SET deleted_at=$1 WHERE id=$2 AND author_id=$3 AND deleted_at IS NULL
    "#)
    .bind(now)
    .bind(post_id)
    .bind(user.id)
    .execute(&db)
    .await
    .map_err(|err| {
        eprintln!("Error deleting post: {:?}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Error deleting post"
        )
    })?;
    
    if pq_res.rows_affected() > 0 {
        Ok((
            StatusCode::CREATED,
            format!("Post id:{} deleted!", post_id)
        ))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST, 
            "Can't delete someone else's post or the post is already deleted or post not found!"
        ))
    }
}
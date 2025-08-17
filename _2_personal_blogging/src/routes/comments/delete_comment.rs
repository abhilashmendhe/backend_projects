use axum::{extract::{Path, State}, http::StatusCode, Extension};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{models::user_model::UserDB, utils::errors::AppError};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostComment {
    pub post_id: i32,
    pub comment_id: i32
}

pub async fn delete_comment(
    Extension(user): Extension<UserDB>,
    State(db): State<PgPool>,
    Path(post_comemnt): Path<PostComment>,
) -> Result<(StatusCode, String), AppError> {

    let pq = sqlx::query(r#"
        DELETE FROM comments WHERE id=$1 AND post_id=$2 AND user_id=$3
    "#)
    .bind(post_comemnt.comment_id)
    .bind(post_comemnt.post_id)
    .bind(user.id)
    .execute(&db)
    .await
    .map_err(|err| {
        eprintln!("Error deleting post comment: {:?}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error deleting post comment! Please try again later!")
    })?;

    if pq.rows_affected() == 0 {
        return Err(
            AppError::new(StatusCode::UNAUTHORIZED, 
                "Can't delete someone's comment and/or please login to delete the comment"
            ));
    }

    Ok((
        StatusCode::CREATED,
        format!("Comment id: {} deleted from the post id: {}", post_comemnt.comment_id, post_comemnt.post_id)
    ))
}

use axum::{extract::{Path, State}, http::StatusCode, Extension};
use sqlx::PgPool;

use crate::{models::user_model::UserDB, routes::comments::{comment_extractor::ValidateCommentPost, delete_comment::PostComment}, utils::errors::AppError};

pub async fn update_comment(
    Extension(user): Extension<UserDB>,
    State(db): State<PgPool>,
    Path(post_comemnt): Path<PostComment>,
    comment: ValidateCommentPost,
) -> Result<(StatusCode, String), AppError> {

    let pq = sqlx::query(r#"
        UPDATE comments SET content=$1 WHERE id=$2 AND post_id=$3 AND user_id=$4
    "#)
    .bind(comment.comment.unwrap())
    .bind(post_comemnt.comment_id)
    .bind(post_comemnt.post_id)
    .bind(user.id)
    .execute(&db)
    .await
    .map_err(|err| {
        eprintln!("Error updating the post comment: {:?}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error updating the post comment! Please try again later!")
    })?;

    if pq.rows_affected() == 0 {
        return Err(
            AppError::new(StatusCode::UNAUTHORIZED, 
                "Can't update someone else's comment and/or please login to update the comment"
            ));
    }

    Ok((
        StatusCode::CREATED,
        format!("Comment id: {} update for the post id: {} by user id: {}", post_comemnt.comment_id, post_comemnt.post_id, user.id)
    ))
}
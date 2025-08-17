use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Extension};
use sqlx::PgPool;

use crate::models::user_model::UserDB;
use crate::routes::comments::comment_extractor::ValidateCommentPost;
use crate::utils::errors::AppError;

pub async fn create_comment(
    Extension(user): Extension<UserDB>,
    State(db): State<PgPool>,
    Path(post_id): Path<i32>,
    comment: ValidateCommentPost,
) -> Result<(StatusCode, String), AppError> {

    let pg = if let Some(parent_comment_id) = comment.parent_comment_id {
        sqlx::query(r#"
            INSERT INTO comments(post_id, user_id, content, parent_comment_id) 
            VALUES ($1, $2, $3, $4)
        "#)
        .bind(post_id)
        .bind(user.id)
        .bind(comment.comment.unwrap())
        .bind(parent_comment_id)
    } else {
        sqlx::query(r#"
            INSERT INTO comments(post_id, user_id, content) 
            VALUES ($1, $2, $3)
        "#)
        .bind(post_id)
        .bind(user.id)
        .bind(comment.comment.unwrap())
    };
    
    pg
    .execute(&db)
    .await
    .map_err(|err|{
        eprintln!("Error while creating comment on post: {:?}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Can't publish comment on post. Please try again later!")
    })?;

    Ok((
        StatusCode::CREATED,
        format!("User-id: {} created comment on post id: {}", user.id, post_id)
    ))
}
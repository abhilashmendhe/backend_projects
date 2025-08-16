use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use sqlx::PgPool;

use crate::{models::user_model::UserDB, routes::posts::{post_extractor::ValidateCreatePost, ResponsePost}, utils::errors::AppError};

pub async fn update_post(
    Extension(user): Extension<UserDB>,
    State(db): State<PgPool>,
    Path(post_id): Path<i32>,
    post: ValidateCreatePost
) -> Result<(StatusCode, Json<ResponsePost>), AppError> {
    println!("In update post by id");
    let login_r = if let Some(login_r) = post.login_required {
        login_r
    } else {
        false
    };
    let pq_res = sqlx::query(r#"
        UPDATE posts SET title=$1, content=$2, published=$3, login_required=$4
        WHERE id=$5 AND author_id=$6 AND deleted_at IS NULL
    "#)
    .bind(post.title.clone().unwrap())
    .bind(post.content.clone().unwrap())
    .bind(post.published.unwrap())
    .bind(login_r)
    .bind(post_id)
    .bind(user.id)
    .execute(&db)
    .await
    .map_err(|err| {
        eprintln!("Error updating post: {:?}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Error updating post"
        )
    })?;
    
    if pq_res.rows_affected() > 0 {
        let post_resp = ResponsePost {
            id: post_id,
            title: post.title.unwrap(),
            content: post.content.unwrap(),
            author_id: user.id,
            published: post.published.unwrap(),
        };
        Ok((
            StatusCode::CREATED,
            Json(post_resp)
        ))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST, 
            "Can't update post or the post does not exists!"
        ))
    }
}
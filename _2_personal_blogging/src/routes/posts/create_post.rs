use axum::{extract::State, http::StatusCode, Extension, Json};
use sqlx::PgPool;
use sqlx::Row;

use crate::{models::user_model::UserDB, routes::posts::{post_extractor::ValidateCreatePost, ResponsePost}, utils::errors::AppError};

pub async fn create_post(
    Extension(user): Extension<UserDB>,
    State(db): State<PgPool>,
    post: ValidateCreatePost
) -> Result<(StatusCode, Json<ResponsePost>), AppError> {
    
    let post_row = if let Some(login_r) = post.login_required {
        sqlx::query(r#"
                INSERT INTO posts(title, content, author_id, published, login_required) 
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, title, content, author_id, created_at, published
            "#)
            .bind(post.title.unwrap())
            .bind(post.content.unwrap())
            .bind(user.id)
            .bind(post.published.unwrap())
            .bind(login_r)
            .fetch_one(&db)
            .await
    } else {
        sqlx::query(r#"
                INSERT INTO posts(title, content, author_id, published) 
                VALUES ($1, $2, $3, $4)
                RETURNING id, title, content, author_id, created_at, published
            "#)
            .bind(post.title.unwrap())
            .bind(post.content.unwrap())
            .bind(user.id)
            .bind(post.published.unwrap())
            .fetch_one(&db)
            .await
    };
    
    let post_row = post_row
    .map_err(|err| {
        eprintln!("Error inserting post in the DB: {:?}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Something went wrong while creating blog post. Please try again later."
        )
    })?;

    let resp_post = ResponsePost {
        id: post_row.get("id"),
        title: post_row.get("title"),
        content: post_row.get("content"),
        author_id: post_row.get("author_id"),
        published: post_row.get("published"),
    };

    Ok((
        StatusCode::CREATED,
        Json(resp_post)
    ))
}
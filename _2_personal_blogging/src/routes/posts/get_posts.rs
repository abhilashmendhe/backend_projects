use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};
use sqlx::PgPool;
use sqlx::Row;

use crate::{routes::posts::{ResponsePost, ResponsePosts}, utils::{config::Config, errors::AppError, jwt::validate_token}};

pub async fn fetch_all_posts(
    State(db): State<PgPool>,
    State(config): State<Config>,
    headers: HeaderMap
) -> Result<(StatusCode, Json<ResponsePosts>), AppError> {
    let all_posts = if let Some(token) = headers.get("x-auth-token") {
        println!("token: {:?}",token);
        let header_token = token.to_str()
            .map_err(|err|{
                eprintln!("Error extracting token: {:?}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
            })?;
        
        validate_token(&config.jwt_secret(), header_token).await?;

        sqlx::query(r#"
            SELECT * FROM posts WHERE published=$1 AND deleted_at IS NULL
        "#)
        .bind(true)
        .fetch_all(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by token: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?        

    } else {
        sqlx::query(r#"
            SELECT * FROM posts WHERE published=$1 AND login_required=$2 AND deleted_at IS NULL
        "#)
        .bind(true)
        .bind(false)
        .fetch_all(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by token: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?
    };

    let res = all_posts.iter().map(|row| ResponsePost {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            published: true
        }).collect::<Vec<ResponsePost>>();

    Ok((StatusCode::OK, Json(ResponsePosts{data: res})))
}

pub async fn fetch_post_by_id() {}
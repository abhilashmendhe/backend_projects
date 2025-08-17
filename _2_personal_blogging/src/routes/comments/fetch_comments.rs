use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;

use crate::{models::user_model::UserDB, utils::errors::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentStruct {
    pub comment_id: i32,
    pub user_id: i32,
    pub comment: String,
    pub parent_comment_id: Option<i32>
}

pub async fn fetch_comments(
    Extension(_): Extension<UserDB>,
    State(db): State<PgPool>,
    Path(post_id): Path<i32>
) -> Result<Json<Vec<CommentStruct>>, AppError> {

    let all_comments = sqlx::query(r#"
    WITH RECURSIVE comment_tree AS (
        SELECT
            id,
            post_id,
            user_id,
            content,
            created_at,
            parent_comment_id,
            1 AS depth
        FROM
            comments
        WHERE
            parent_comment_id IS NULL -- Start with top-level comments

        UNION ALL

        SELECT
            c.id,
            c.post_id,
            c.user_id,
            c.content,
            c.created_at,
            c.parent_comment_id,
            ct.depth + 1 AS depth
        FROM
            comments c
        JOIN
            comment_tree ct ON c.parent_comment_id = ct.id
    )
    SELECT * FROM comment_tree WHERE post_id=$1 ORDER BY post_id, created_at, depth;
    "#)
    .bind(post_id)
    .fetch_all(&db)
    .await
    .map_err(|err| {
        eprintln!("Error fetching all comments: {:?}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error fetching comments. Please try again later!")
    })?;

    let comments = all_comments
                            .iter()
                            .map(|row| CommentStruct{
                                comment_id: row.get("id"),
                                comment: row.get("content"),
                                user_id: row.get("user_id"),
                                parent_comment_id: row.get("parent_comment_id"),
                                
                            })
                            .collect::<Vec<_>>();

    Ok(Json(comments))
}
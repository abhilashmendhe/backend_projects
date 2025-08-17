use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CommentDB {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub content: String,
    pub parent_comment_id: i32,
    pub created_at: DateTime<Utc>,
}
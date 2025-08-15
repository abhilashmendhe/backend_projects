use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PostDB {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub published: bool
}
use serde::Serialize;

pub mod create_post;
pub mod post_extractor;
pub mod get_posts;

#[derive(Debug, Serialize)]
pub struct ResponsePost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub published: bool
}

#[derive(Debug, Serialize)]
pub struct ResponsePosts {
    pub data: Vec<ResponsePost>
}
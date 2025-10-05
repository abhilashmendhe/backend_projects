use serde::Serialize;

pub mod get_health;

#[derive(Debug, Serialize)]
pub struct Health {
    message: String,
    visit_count: usize
}
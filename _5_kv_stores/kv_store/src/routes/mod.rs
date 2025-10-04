use serde::Serialize;

pub mod health;

#[derive(Debug, Serialize)]
pub struct Health {
    message: String,
    visit_count: usize
}
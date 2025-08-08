use serde::{Deserialize, Serialize};

pub mod create_task;
pub mod create_task_extractor;
pub mod get_all_tasks;

#[derive(Serialize, Deserialize)]
pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask
}

#[derive(Deserialize, Serialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>
}
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::users::Model;
use crate::routes::tasks::{ResponseDataTask, ResponseTask};
use crate::utils::app_error::AppError;
use crate::utils::app_state::AppState;

use crate::database::tasks::{self, Entity as Tasks};

pub async fn get_one_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>
) -> Result<Json<ResponseDataTask>, AppError> {
    let task =  Tasks::find_by_id(task_id)
            .filter(tasks::Column::UserId.eq(Some(user.id)))
            .one(&state.db)
            .await
            .map_err(|err| {
                eprintln!("Error retrieving task for id: {}, err: {:?}", task_id, err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error retrieving task.")
            })?;
    if let Some(task) = task {
        
        let resp_task = ResponseTask{
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            completed_at: task.completed_at.map(|completed_at| completed_at.to_string()),
        };
        Ok(Json(ResponseDataTask { data: resp_task }))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "We couldn't find your task!!"))
    }
}
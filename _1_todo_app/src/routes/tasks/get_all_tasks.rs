use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::users::Model as UserModel;
use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;
use crate::routes::tasks::{ResponseDataTasks, ResponseTask};
use crate::utils::app_error::AppError;
use crate::utils::app_state::AppState;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(state): State<AppState>, 
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .all(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error retrieving tasks: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
        })?
        .into_iter()
        .map(|db_task| ResponseTask{
            id: db_task.id,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
            completed_at: if let Some(completed_at) = db_task.completed_at {
                            Some(completed_at.to_string())
                        } else {
                            None
                        },
        })
        .collect::<Vec<ResponseTask>>();
    Ok(Json(ResponseDataTasks{data: tasks}))
}

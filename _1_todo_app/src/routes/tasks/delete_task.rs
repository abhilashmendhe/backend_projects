use axum::{extract::{Path, State}, http::StatusCode, Extension};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};

use crate::{database::tasks::{self, Model}, utils::{app_error::AppError, app_state::AppState}};
use crate::database::tasks::Entity as Tasks;

pub async fn soft_delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
) -> Result<(), AppError> {

    let task = Tasks::find_by_id(task_id)
            .filter(tasks::Column::UserId.eq(Some(user.id)))
            .one(&state.db)
            .await
            .map_err(|err| {
                eprintln!("Error deleting task: {:?}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error deleting the task")
            })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, 
            "Task not found to be deleted!"))
    };
    let now = Utc::now();
    task.deleted_at = Set(Some(now.into()));
    task.save(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error saving after soft-deleting: {:?}",err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 
            "Error deleting task!")
        })?;
    Ok(())
}
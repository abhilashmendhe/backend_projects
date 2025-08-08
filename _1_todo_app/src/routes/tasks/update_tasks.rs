use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};

use crate::{database::tasks::{self, Entity as Tasks}, routes::tasks::RequestTask};
use crate::{database::users::Model, utils::{app_error::AppError, app_state::AppState}};

pub async fn mark_completed(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>
) -> Result<(), AppError> {
    println!("At mark completed!");
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error retrieving task for update: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happened")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));
    task.save(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error making task as completed: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error while updating `compated at`")
        })?;
    
    Ok(())
}

pub async fn mark_uncompleted(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>
) -> Result<(), AppError> {
    // println!("At mark completed!");
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error retrieving task for update: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happened")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    task.completed_at = Set(None);
    task.save(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error making task as completed: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error while updating `compated at`")
        })?;
    
    Ok(())
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
    Json(request_task): Json<RequestTask>
) -> Result<(), AppError> {
    // println!("At mark completed!");
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error retrieving task for update: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happened")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

     if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }

    if let Some(title) = request_task.title {
        task.title = Set(title);
    }

    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    if let Some(description) = request_task.description {
        task.description = Set(description);
    }
    
    task.save(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error making task as completed: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error while updating `compated at`")
        })?;
    
    Ok(())
}
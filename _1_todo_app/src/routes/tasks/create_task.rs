use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::ActiveValue::Set;

use crate::database::users::Model as UserModel;
use crate::database::tasks;
use crate::routes::tasks::create_task_extractor::ValidateCreateTask;
use crate::routes::tasks::ResponseDataTask;
use crate::{routes::tasks::ResponseTask, utils::{app_error::AppError, app_state::AppState}};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(state): State<AppState>,
    // Json(request_task): Json<RequestTask> 
    task: ValidateCreateTask
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    // println!("In create task");
    let new_task = tasks::ActiveModel{
        priority: Set(task.priority),
        title: Set(task.title.unwrap().clone()),
        description: Set(task.description.clone()),
        user_id: Set(Some(user.id)),
        is_default: Set(None),
        ..Default::default()
    };

    // save to db
    let task = new_task
        .save(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error creating a new task: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Error creating task")
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting task after creating: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
        })?;
    
    // Create a response task
    let response = ResponseTask {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok((
        StatusCode::CREATED,
        Json(ResponseDataTask { data: response }),
    )) 
}
use axum::{middleware, routing::{get, post}, Router};

use crate::{middleware::require_auth::require_auth, routes::{hello_world::hello_world, tasks::{create_task::create_task, get_all_tasks::get_all_tasks}, users::{create_user::create_user, login::login, logout::logout}}, utils::app_state::AppState};

pub fn create_router(app_state: AppState) -> Router {
   
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route_layer(middleware::from_fn_with_state(app_state.clone(),require_auth))
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login",   post(login))
        .with_state(app_state)
}

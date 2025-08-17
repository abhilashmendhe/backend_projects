use std::sync::{atomic::AtomicU32, Arc};

use axum::{extract::State, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, patch, post, put}, Router};

use crate::{middleware::require_auth::require_authentication, routes::{comments::{create_comment::create_comment, delete_comment::delete_comment, update_comment::update_comment}, posts::{create_post::create_post, delete_post::delete_post, get_posts::{fetch_all_posts, fetch_post_by_id}, update_post::update_post}, users::{create_user::create_user, delete_user::delete_user, login::login, logout::logout, update_password::update_user_password}}, utils::app_state::AppState};

pub fn create_router(app_state: AppState) -> Router {

    Router::new()

        
        
        .route("/v1/posts", get(fetch_all_posts))
        .route("/v1/posts/{:post_id}", get(fetch_post_by_id))

        .route("/v1/posts/{post_id}/comments/{comment_id}", patch(update_comment)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication)))

        .route("/v1/posts/{post_id}/comments/{comment_id}", delete(delete_comment)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication)))

        .route("/v1/posts/{:post_id}/comments", post(create_comment)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication)))

        .route("/v1/posts/{:post_id}", patch(update_post)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication)))

        .route("/v1/posts/{:post_id}", delete(delete_post)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication)))

        .route("/v1/posts", post(create_post)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication)))

        .route("/v1/users/logout", post(logout)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication)))
        
        .route("/v1/users", delete(delete_user)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication)))
            
        .route("/v1/users/login", post(login))
        .route("/v1/users", put(update_user_password))
        .route("/v1/users", post(create_user))
        .route("/health", get(health_check))
        .with_state(app_state)
}

pub async fn health_check(
    State(count): State<Arc<AtomicU32>>
) -> impl IntoResponse {
    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let value = count.load(std::sync::atomic::Ordering::SeqCst);
    (StatusCode::OK, format!("Works! Visit count: {}\n", value))
}
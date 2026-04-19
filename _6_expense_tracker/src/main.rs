use _6_expense_tracker::{run, utils::app_state::AppState};
use actix_web::web;
use tracing::level_filters::LevelFilter;

#[actix_web::main]
async fn main() -> Result<(), _6_expense_tracker::utils::errors::ExpenseTrackerErr> {
    // 1. enable tracing.
    tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();

    let app_state = web::Data::new(AppState::new());

    run(app_state).await?;
    Ok(())
}


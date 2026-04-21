use _6_expense_tracker::{run, utils::{app_state::AppState, config::Config, errors::ExpenseTrackerErr}};
use actix_web::web;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::level_filters::LevelFilter;

#[actix_web::main]
async fn main() -> Result<(), ExpenseTrackerErr> {
    // 1. enable tracing.
    tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();

    // 2. Load .env variables
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let secret = std::env::var("SECRET")?;

    // 3. Initialize config
    let config = Config::new(db_url, secret);
    
    // 4. Connect to DB
    let pool = match PgPoolOptions::new().max_connections(4).connect(&config.db_url()).await {
        Ok(pool) => {
            println!("✅ Connection to DB is succesfull!");
            pool
        },
        Err(err) => {
            println!("🔥 Failed to connect to DB: {:?}", err);
            std::process::exit(1);
        },
    };

    let app_state = web::Data::new(
        AppState::new(config, pool)
    );

    run(app_state).await?;
    Ok(())
}


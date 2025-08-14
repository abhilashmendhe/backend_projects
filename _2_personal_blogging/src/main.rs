/*
To run:
    $ cargo watch -q -c -w src/ -x run
*/

use _2_personal_blogging::utils::app_state::AppState;
use _2_personal_blogging::utils::config::Config;
use _2_personal_blogging::utils::errors::BlogAppError;
use _2_personal_blogging::run;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), BlogAppError> {
    // 1. enable tracing.
    tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();

    // 2. load vars from .env
    dotenv::dotenv().ok();
    
    // 3. init config
    let config = Config::new();

    // 4. connect to database
    let pool = match PgPoolOptions::new().max_connections(10).connect(&config.db_url()).await {
        Ok(pool) => {
            println!("✅ Connection to DB is succesfull!");
            pool
        },
        Err(err) => {
            println!("🔥 Failed to connect to DB: {:?}", err);
            std::process::exit(1);
        },

    };
    let app_state = AppState::new(config, pool);
    
    run(app_state).await?;
    Ok(())
}

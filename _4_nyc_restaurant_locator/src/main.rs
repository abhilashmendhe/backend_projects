use _4_nyc_restaurant_locator::{run, utils::{app_state::AppState, errors::WebError}};
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info, Level};

#[tokio::main]
async fn main() -> Result<(), WebError> {

    // 1. Init tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // 2. load vars from .env file
    dotenv::dotenv().ok();

    // 3. read vars from .env
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}",host,port);

    // 4. set up postgres conn
    let db_url = std::env::var("DB_URL").expect("DB_URL should be present in .env file!");

    let conn = loop {match PgPoolOptions::new()
                .max_connections(10)
                .connect(&db_url)
                .await {
                    Ok(pool) => {
                        info!("✅ Connection to PostGIS DB is succesfull!");
                        break pool
                    },
                    Err(err) => {
                        error!("🔥 Failed to connect to DB: {:?}", err);
                        let _ = std::thread::sleep(std::time::Duration::from_millis(5000));
                    }
                };
            };
    
    let app_state = AppState::new(conn);

    run(
        address, 
        app_state
    ).await
}

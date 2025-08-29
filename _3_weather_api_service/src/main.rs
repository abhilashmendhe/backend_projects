/*
$ cargo watch -q -c -w src/ -x run
*/

use _3_weather_api_service::{run, utils::{app_state::AppState, config::Config, errors::WeatherServiceErr}};
use tracing::*;

#[tokio::main]  
async fn main() -> Result<(), WeatherServiceErr> {

    // 1. Init tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // 2. load vars from .env
    dotenv::dotenv().ok();

    // 3. read vars from .env
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("{}:{}", host, port);

    let redis_port = std::env::var("REDIS_PORT").unwrap_or("6379".to_string());
    let redis_addr = format!("redis://{}:{}/",host,redis_port);

    let web_api_key = std::env::var("WEB_API_KEY")?;
    
    // 4. Init Config
    let config = Config::new(web_api_key);

    let client = redis::Client::open(redis_addr)  ?;
    let conn;

    loop {
        match client.get_multiplexed_tokio_connection().await {
            Ok(connection) => {
                conn = connection;
                break;
            },
            Err(err) => {
                error!("{:?}",err);
                let _ = std::thread::sleep(std::time::Duration::from_millis(5000));
            },
        }
    }

    // 5. Init AppState
    let app_state = AppState::new(config, conn);

    // 6. run the server
    run(&addr, app_state).await?;

    Ok(())

}

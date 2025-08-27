/*
$ cargo watch -q -c -w src/ -x run
*/

use _3_weather_api_service::{run, utils::{app_state::AppState, config::Config, errors::WeatherServiceErr}};


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

    let web_api_key = std::env::var("WEB_API_KEY")?;
    
    // 4. Init Config
    let config = Config::new(web_api_key);

    // 5. Init AppState
    let app_state = AppState::new(config);

    // 6. run the server
    run(&addr, app_state).await?;

    Ok(())

}

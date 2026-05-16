use actix_web::web;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing::level_filters::LevelFilter;
use tu_server::{
    run,
    utils::{app_state::AppState, config::Config, errors::TinyUrlError},
};

#[derive(Parser, Debug)]
struct ServerCli {
    #[arg(short, long)]
    localhost: String,

    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(short, long, default_value_t = 4)]
    server_workers: usize,

    #[arg(short, long, default_value_t = 4)]
    db_conn_workers: u32,
}

#[actix_web::main]
async fn main() -> Result<(), TinyUrlError> {
    // let uuid = uuid7::uuid7();
    // let uuid_bytes = uuid.as_bytes();
    // // println!("{:?}", uuid);
    // println!("{}", u128::from_be_bytes(*uuid_bytes));

    // 0. Get args
    let scli = ServerCli::parse();
    let localhost = scli.localhost;
    let port = scli.port;
    let db_conn_workers = scli.db_conn_workers;
    let server_workers = scli.server_workers;

    // 1. Enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    // 2. Load .env variables
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let jwt_secret = std::env::var("SECRET")?;

    // 3. Init config
    let config = Config::new(db_url, jwt_secret, db_conn_workers);

    // 4. Connect to DB
    let pool = match PgPoolOptions::new()
        .max_connections(config.db_conn_workers())
        .connect(&config.db_url())
        .await
    {
        Ok(pool) => {
            tracing::info!("✅ Connection to DB is succesfull!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to DB: {:?}", err);
            std::process::exit(1);
        }
    };

    // 5. Init app state
    let app_state = web::Data::new(AppState::new(config, pool));

    // 6. Run server
    run(localhost, port, server_workers, app_state).await?;
    Ok(())
}

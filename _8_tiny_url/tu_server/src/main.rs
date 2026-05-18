use actix_web::web;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing::level_filters::LevelFilter;
use tu_server::{
    run,
    utils::{app_state::AppState, config::Config, errors::TinyUrlError},
};

/**
 * To run: cargo watch -q -c -w src/ -x "run -- -l localhost"
 * To run: cargo watch -q -c -w src/ -x "run -- --addr localhost -p 8080 --redis-addr localhost --redis-port 6379"
 */

#[derive(Parser, Debug)]
struct ServerCli {
    #[arg(short, long)]
    addr: String,

    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(long)]
    redis_addr: String,

    #[arg(long, default_value_t = 6379)]
    redis_port: u16,

    #[arg(short, long, default_value_t = 4)]
    server_workers: usize,

    #[arg(short, long, default_value_t = 4)]
    db_conn_workers: u32,
}

#[actix_web::main]
async fn main() -> Result<(), TinyUrlError> {
    // 0. Get args
    let scli = ServerCli::parse();
    let localhost = scli.addr;
    let port = scli.port;
    let redis_localhost = scli.redis_addr;
    let redis_port = scli.redis_port;
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

    // 5. Check with redis connection
    let redis_addr = format!("redis://{}:{}/", redis_localhost, redis_port);
    let redis_client = redis::Client::open(redis_addr)?;
    let redis_conn;
    loop {
        match redis_client.get_multiplexed_async_connection().await {
            Ok(connection) => {
                redis_conn = connection;
                break;
            }
            Err(err) => {
                tracing::error!("{:?}", err);
                let _ = std::thread::sleep(std::time::Duration::from_millis(5000));
            }
        }
    }

    // 5. Init app state
    let app_state = web::Data::new(AppState::new(config, pool, redis_conn));

    // 6. Run server
    run(localhost, port, server_workers, app_state).await?;
    Ok(())
}

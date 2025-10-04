use tokio::net::TcpListener;

use crate::{routers::routers, utils::{app_state::AppState, errors::KVError}};

pub mod routers;
pub mod utils;
pub mod data;
pub mod routes;

pub async fn run(app_state: AppState, addr: &str) -> Result<(), KVError>{
    
    let app = routers(app_state);
    
    let listener = TcpListener::bind(addr).await?;
    println!("🚀 Timestore KV Started listening on `{}`\n", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
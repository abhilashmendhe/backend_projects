use tokio::net::TcpListener;

use crate::{routers::routers, utils::{app_state::AppState, errors::KVError}};

pub mod routers;
pub mod utils;

pub async fn run(app_state: AppState, addr_ip: &str) -> Result<(), KVError>{
    
    let app = routers(app_state);
    
    let listener = TcpListener::bind(addr_ip).await?;

    axum::serve(listener, app).await?;
    
    Ok(())
}
use redis::{aio::MultiplexedConnection, AsyncCommands};

use crate::utils::errors::WeatherServiceErr;

pub async fn geoadd(
    mut conn: MultiplexedConnection,
    city_name: String,
    lat: f64,
    long: f64
) -> Result<(), WeatherServiceErr> {

    conn.geo_add::<String,(String,String,String),String>("location".to_string(), (
        lat.to_string(), 
        long.to_string(),
        city_name)).await?;
    
    Ok(())
}
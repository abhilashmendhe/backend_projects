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
        city_name.to_lowercase())).await?;
    
    // now check the ttl value of key location
    let ttl = conn.ttl::<&str,i64>("location").await?;
    // println!("ttl:{}",ttl);
    // add a ttl value if less than 0
    if ttl < 0 {
        conn.expire::<&str, i64>("location", 600).await?;
    } 
    Ok(())
}
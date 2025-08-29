use redis::{aio::MultiplexedConnection, AsyncCommands};

use crate::utils::errors::WeatherServiceErr;

pub async fn insert_kv(
    mut conn: MultiplexedConnection,
    city_name: String,
    api_weather: String,
) -> Result<(), WeatherServiceErr> {
    conn.set_ex::<String,String,String>(city_name.clone(), api_weather, 600).await?;
    Ok(())
}
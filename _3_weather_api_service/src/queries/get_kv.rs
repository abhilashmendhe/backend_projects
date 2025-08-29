use axum::Json;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use reqwest::StatusCode;
use tracing::info;

use crate::models::weather_api_model::WeatherAPIModel;

pub async fn get_kv(
    mut conn: MultiplexedConnection,
    city: String
) {

    match conn   
        // 1.2.1 first check if redis contains the key
        .get::<&str, String>(&city)
        .await {
            Ok(api_weather) => {
                let api_weather_json = serde_json::from_str::<WeatherAPIModel>(&api_weather)?;
                info!("Cached request served for the city: {}",city);
                return Ok((
                    StatusCode::OK,
                    Json(api_weather_json)
                ));
            },
            Err(_) => city,
    }
}
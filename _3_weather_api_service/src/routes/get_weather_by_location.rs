
use axum::{extract::State, Json};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use reqwest::StatusCode;
use tracing::*;

use crate::{ models::weather_api_model::WeatherAPIModel, routes::location_extractor::{QueryParam, ValidateLoc}, utils::{config::Config, errors::{WeatherServiceErr, WebServerErr}}, weather_api_requests::get_today_weather::get_today_weather};



pub async fn get_weather_by_location(
    State(config): State<Config>,
    State(mut conn): State<MultiplexedConnection>,
    ValidateLoc(param): ValidateLoc<QueryParam>,
) -> Result<(StatusCode, Json<WeatherAPIModel>), WeatherServiceErr> {

    // 1. extract query param

    // 1.1. extract unit metric
    let unit = if let Some(unit) = param.unit {
        if unit.ne("uk") || unit.ne("us") || unit.ne("metric") {
            return Err(
                WeatherServiceErr::WebServerErr(WebServerErr::new(
                    StatusCode::BAD_REQUEST, 
                    "Metric should be either `uk` or `us` or `metric`"
                ))
            );
        } else {
            unit
        }
    } else {
        "uk".to_string()
    };

    // 1.2. extract location (cityname or (lat,long))
    let city_name = match param.location {
        super::location_extractor::Loc::TEXT(city) => {
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
        },
        super::location_extractor::Loc::NUM((lat,long)) => format!("{},{}",lat,long),
    };
    
    
    // 3. If not, then fetch from the api
    let api_weather = get_today_weather(
            config.web_api_key(), 
            city_name.clone(), 
            unit
        ).await
        .map_err(|err| {
            error!("{:?}",err);
            WeatherServiceErr::WebServerErr(WebServerErr::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Failed to decode/fetch the weather result"))
        })?;
    
    // 4. Convert text to json
    let api_weather_json = serde_json::from_str::<WeatherAPIModel>(&api_weather)?;

    // 5. Now save it in the redis-db
    conn.set_ex::<String,String,String>(city_name.clone(), api_weather.clone(), 600).await?;
    conn.geo_add::<String,(String,String,String),String>("location".to_string(), (
        api_weather_json.latitude.to_string(), 
        api_weather_json.longitude.to_string(),
        city_name.clone())).await?;
    info!("Fresh request served for the city: {}",city_name);

    Ok((
        StatusCode::OK,
        Json(api_weather_json)
    ))
}

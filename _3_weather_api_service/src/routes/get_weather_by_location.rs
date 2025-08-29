
use axum::{extract::State, Json};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use reqwest::StatusCode;
use tracing::*;

use crate::{ api_requests::{get_today_weather::get_today_weather, reverse_geo_coding::reverse_geocoding}, models::{geofy_reverse_api_model::GeofyAPIModel, weather_api_model::WeatherAPIModel}, queries::{geoadd::geoadd, georadius::geo_radius, insert_kv::insert_kv}, routes::location_extractor::{QueryParam, ValidateLoc}, utils::{config::Config, errors::{WeatherServiceErr, WebServerErr}}};



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
        super::location_extractor::Loc::NUM((lat,long)) => {
        // 1.2.1 first check if redis contains the key
            let result = geo_radius(
                conn.clone(), 
                lat, 
                long
            ).await?;
            if result.len() > 0 {
                let found_city = result[0].name.clone();
                let api_weather = conn   

                .get::<&str, String>(&found_city)
                .await?;
                let api_weather_json = serde_json::from_str::<WeatherAPIModel>(&api_weather)?;
                info!("Cached request served for the city coordinates: {}",found_city);
                return Ok((
                    StatusCode::OK,
                    Json(api_weather_json)
                ));
            } else {
                // now do reverse geo lookup for the city name
                info!("Now do reverse geo lookup");
                
                let geofy_loc_info = reverse_geocoding(
                    lat, 
                    long, 
                    config.geofy_key()
                ).await?;
                let geofy_loc_json = serde_json::from_str::<GeofyAPIModel>(&geofy_loc_info)?;
                
                let found_city = if geofy_loc_json.results.len() > 0 {
                    let results = geofy_loc_json.results;
                    let first_result = results[0].clone();
                    first_result.city
                } else {
                    return Err(WeatherServiceErr::WebServerErr(WebServerErr::new(
                        StatusCode::INTERNAL_SERVER_ERROR, 
                        "Failed to decode/fetch the weather result")));
                };
                // "berlin".to_string()
                found_city
            }
        },
    };
    
    
    // 3. If not, then fetch from the api
    let api_weather = get_today_weather(
            config.weather_api_key(), 
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
    // 5.1 create key-value of location -> JSON
    insert_kv(
        conn.clone(), 
        city_name.clone(), 
        api_weather,
    ).await?;    
    // 5.2 add a geo json lat-long value to the location key
    geoadd(
        conn, 
        city_name.clone(), 
        api_weather_json.latitude, 
        api_weather_json.longitude
    ).await?;

    info!("Fresh request served for the city: {}",city_name);

    Ok((
        StatusCode::OK,
        Json(api_weather_json)
    ))
}


use axum::{extract::State, Json};
use reqwest::StatusCode;
use tracing::error;

use crate::{ models::weather_api_model::WeatherAPIModel, routes::location_extractor::{QueryParam, ValidateLoc}, utils::{config::Config, errors::WebServerErr}, weather_api_requests::get_today_weather::get_today_weather};



pub async fn get_weather_by_location(
    State(config): State<Config>,
    // Query(params): Query<QueryParam>
    ValidateLoc(param): ValidateLoc<QueryParam>
) -> Result<(StatusCode, Json<WeatherAPIModel>), WebServerErr> {

    // 1. extract query param
    // 1.1. extract location (cityname or (lat,long))
    let loc_str = match param.location {
        super::location_extractor::Loc::TEXT(city) => city,
        super::location_extractor::Loc::NUM((lat,long)) => format!("{},{}",lat,long),
    };
    // 1.2. extract unit metric
    let unit = if let Some(unit) = param.unit {
        if unit.ne("uk") || unit.ne("us") || unit.ne("metric") {
            return Err(
                WebServerErr::new(
                    StatusCode::BAD_REQUEST, 
                    "Metric should be either `uk` or `us` or `metric`"
                )
            );
        } else {
            unit
        }
    } else {
        "uk".to_string()
    };

    // 2. first check if redis contains the key

    // 3. If not, then fetch from the api
    let api_weather = get_today_weather(
            config.web_api_key(), 
            loc_str, 
            unit
        ).await
        .map_err(|err| {
            error!("{:?}",err);
            WebServerErr::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Failed to decode/fetch the weather result")
        })?;

    // dbg!(api_weather);
    Ok((
        StatusCode::OK,
        Json(api_weather)
    ))
}

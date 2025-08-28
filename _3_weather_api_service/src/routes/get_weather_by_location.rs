
use axum::extract::{ State};

use crate::{ routes::location_extractor::{QueryParam, ValidateLoc}, utils::{config::Config, errors::WebServerErr}, weather_api_requests::get_today_weather::get_today_weather};



pub async fn get_weather_by_location(
    State(config): State<Config>,
    // Query(params): Query<QueryParam>
    ValidateLoc(param): ValidateLoc<QueryParam>
) -> Result<(), WebServerErr> {

    // 0. extract location param
    let loc_str = match param.location {
        super::location_extractor::Loc::TEXT(city) => city,
        super::location_extractor::Loc::NUM((lat,long)) => format!("{},{}",lat,long),
    };

    // 1. first check if redis contains the key

    // 2. If not, then fetch from the api
    get_today_weather(config.web_api_key(), loc_str)
    .await?;

    Ok(())
}

use reqwest::StatusCode;
use tracing::error;

use crate::{utils::errors::{WeatherServiceErr, WebServerErr}};

pub async fn get_today_weather(
    web_api_key: String, 
    location: String, 
    unit: String
) -> Result<String, WeatherServiceErr> {

    let url = format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{location}/today?unitGroup={unit}&key={web_api_key}&contentType=json");
    let result = reqwest::get(url)
        .await
        .map_err(|err|{
            error!("{:?}",err);
            WeatherServiceErr::WebServerErr(WebServerErr::new(StatusCode::NOT_FOUND, "Failed to get the weather for the specified location"))
        })?;    
    Ok(result.text().await?)
}
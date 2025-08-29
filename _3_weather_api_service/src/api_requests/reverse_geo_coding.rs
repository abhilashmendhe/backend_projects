use crate::utils::errors::{WeatherServiceErr, WebServerErr};
use reqwest::StatusCode;
use tracing::error;

pub async fn reverse_geocoding(
    lat: f64,
    long: f64,
    geofy_api_key: String

) -> Result<String, WeatherServiceErr> {
    let url = format!("https://api.geoapify.com/v1/geocode/reverse?lat={lat}&lon={long}&format=json&apiKey={geofy_api_key}");
    let result = reqwest::get(url)
        .await
        .map_err(|err|{
            error!("{:?}",err);
            WeatherServiceErr::WebServerErr(
                WebServerErr::new(
                    StatusCode::NOT_FOUND, 
                    "Failed to get the weather for the specified location"
                ))
        })?;    
    Ok(result.text().await?)   
}
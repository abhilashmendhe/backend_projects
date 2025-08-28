use reqwest::StatusCode;
use tracing::error;

use crate::utils::errors::WebServerErr;

pub async fn get_today_weather(web_api_key: String, location: String) -> Result<(), WebServerErr> {

    let url = format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{location}/today?unitGroup=us&key={web_api_key}&contentType=json");
    let result = reqwest::get(url)
        .await
        .map_err(|err|{
            error!("{:?}",err);
            WebServerErr::new(StatusCode::NOT_FOUND, "Failed to get the weather for specified location")
        })?;
    println!("{:?}", result.text().await);

    Ok(())
}
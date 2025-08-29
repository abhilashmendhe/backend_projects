use chrono::NaiveTime;
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherAPIModel {
    pub queryCost: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub resolvedAddress: String,
    pub address: String,
    pub timezone: String,
    pub tzoffset: f64,
    pub description: String,
    pub currentConditions: CurrentConditions
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentConditions {
    pub datetime:        NaiveTime,
    pub datetimeEpoch:   u128,
    pub temp:            Option<f64>,
    pub feelslike:       Option<f64>,
    pub humidity:        Option<f64>,
    pub dew:             Option<f64>,
    pub precip:          Option<f64>,
    pub precipprob:      Option<f64>,
    pub snow:            Option<f64>,
    pub snowdepth:       Option<f64>,
    pub preciptype:      Option<Vec<String>>,
    pub windgust:        Option<f64>,
    pub windspeed:       Option<f64>,
    pub winddir:         Option<f64>,
    pub pressure:        Option<f64>,
    pub visibility:      Option<f64>,
    pub cloudcover:      Option<f64>,
    pub solarradiation:  Option<f64>,
    pub solarenergy:     Option<f64>,
    pub uvindex:         Option<f64>,
    pub conditions:      String,
    pub icon:            String,
    pub stations:        Vec<String>,
    pub source:          String,
    pub sunrise:         NaiveTime,
    pub sunriseEpoch:    u128,
    pub sunset:          NaiveTime,
    pub sunsetEpoch:     u128,
    pub moonphase:       Option<f64>
}
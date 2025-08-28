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
    pub datetime: NaiveTime,
    pub datetimeEpoch: u128,
    pub temp: f64,
    pub feelslike: f64,
    pub humidity: f64,
    pub dew: f64,
    pub precip: f64,
    pub precipprob: f64,
    pub snow: f64,
    pub snowdepth: f64,
    pub preciptype:      Option<Vec<String>>,
    pub windgust:        Option<f64>,
    pub windspeed:       f64,
    pub winddir:         f64,
    pub pressure:        f64,
    pub visibility:      f64,
    pub cloudcover:      f64,
    pub solarradiation:  f64,
    pub solarenergy:     f64,
    pub uvindex:         f64,
    pub conditions:      String,
    pub icon:            String,
    pub stations:        Vec<String>,
    pub source:          String,
    pub sunrise:         NaiveTime,
    pub sunriseEpoch:    u128,
    pub sunset:          NaiveTime,
    pub sunsetEpoch:     u128,
    pub moonphase:       f64
}
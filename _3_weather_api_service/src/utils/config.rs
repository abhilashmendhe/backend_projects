#[derive(Debug, Clone)]
pub struct Config {
    weather_api_key: String,
    geofy_key: String
}

impl Config {
    pub fn new(weather_api_key: String, geofy_key: String) -> Self {
        Self { weather_api_key, geofy_key }
    }

    pub fn weather_api_key(&self) -> String {
        self.weather_api_key.clone()
    }

    pub fn geofy_key(&self) -> String {
        self.geofy_key.clone()
    }
}

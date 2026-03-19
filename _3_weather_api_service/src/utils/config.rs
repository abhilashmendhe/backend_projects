#[derive(Debug, Clone)]
pub struct Config {
    weather_api_key: String,
    geofy_key: String,
    rate_limit_requests: u32,
    geo_radius: f64
}

impl Config {
    pub fn new(weather_api_key: String, geofy_key: String, rate_limit_requests: u32, geo_radius: f64) -> Self {
        Self { weather_api_key, geofy_key, rate_limit_requests, geo_radius }
    }

    pub fn weather_api_key(&self) -> String {
        self.weather_api_key.clone()
    }

    pub fn geofy_key(&self) -> String {
        self.geofy_key.clone()
    }

    pub fn rate_limit_requests(&self) -> u32 {
        self.rate_limit_requests
    }

    pub fn geo_radius(&self) -> f64 {
        self.geo_radius
    }
}

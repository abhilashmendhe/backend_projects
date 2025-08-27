#[derive(Debug, Clone)]
pub struct Config {
    web_api_key: String
}

impl Config {
    pub fn new(web_api_key: String) -> Self {
        Self { web_api_key }
    }

    pub fn web_api_key(&self) -> String {
        self.web_api_key.clone()
    }
}

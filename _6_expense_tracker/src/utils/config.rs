#[derive(Debug)]
pub struct Config {
    db_url: String,
    secret: String
}

impl Config {
    pub fn new(db_url: String, secret: String) -> Self {
        Config { db_url, secret }
    }
    pub fn db_url(&self) -> &str {
        &self.db_url
    }
    pub fn secret(&self) -> &str {
        &self.secret
    }
}
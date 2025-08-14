pub struct Config {
    db_url: String,
    jwt_secret: String
}

impl Config {
    pub fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let jwt_secret = std::env::var("JWT_SECRET").unwrap();
        // println!("db_url: {}", db_url);
        Self {
            db_url,
            jwt_secret
        }
    }

    pub fn db_url(&self) -> String {
        self.db_url.clone()
    }

    pub fn jwt_secret(&self) -> String {
        self.jwt_secret.clone()
    }
}
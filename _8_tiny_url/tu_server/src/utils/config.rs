#[derive(Debug)]
pub struct Config {
    db_url: String,
    jwt_secret: String,
    db_conn_workers: u32,
}

impl Config {
    pub fn new(db_url: String, jwt_secret: String, db_conn_workers: u32) -> Self {
        Self {
            db_url,
            jwt_secret,
            db_conn_workers,
        }
    }
    pub fn db_url(&self) -> String {
        self.db_url.to_string()
    }
    pub fn jwt_secret(&self) -> String {
        self.jwt_secret.to_string()
    }
    pub fn db_conn_workers(&self) -> u32 {
        self.db_conn_workers
    }
}

use serde::Deserialize;

pub mod get_value;
pub mod put_value;


#[derive(Debug, Deserialize)]
pub struct GetRequestKV {
    pub key: String,
    pub timestamp: u64
}

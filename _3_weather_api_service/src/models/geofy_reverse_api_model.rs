use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeofyAPIModel {
    pub results: Vec<GeofyResult>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeofyResult {
    pub country:         String,
    pub city:            String,
    pub postcode:        String,
    pub lon:             f64,
    pub lat:             f64,
    pub state:           String,
    pub state_code:      String,
    pub formatted:       String,
    pub address_line1:   String,
    pub address_line2:   String,
    pub plus_code:       String,
}
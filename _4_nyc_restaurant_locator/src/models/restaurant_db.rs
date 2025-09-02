use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, types::Json};

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "jsonb")] 
pub struct DetailedRatings {
    #[serde(rename = "Rating 1")]
    pub rating1: Option<f64>,

    #[serde(rename = "Rating 2")]
    pub rating2: Option<f64>,

    #[serde(rename = "Rating 3")]
    pub rating3: Option<f64>,

    #[serde(rename = "Rating 4")]
    pub rating4: Option<f64>,

    #[serde(rename = "Rating 5")]
    pub rating5: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Location {
    pub srid: i32,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize,sqlx::FromRow)]
pub struct RestaurantDB {
    pub id:               i32,
    pub url:              String,
    pub name:             String,
    pub rating:           f64,
    pub rating_count:     i32,
    pub detailed_ratings: Json<DetailedRatings>,
    pub price_category:   i32,
    pub address:          String,
    pub location:         Json<Location>,
    pub zipcode:          i32
}
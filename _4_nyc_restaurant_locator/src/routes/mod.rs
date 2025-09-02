use serde::Serialize;

use crate::models::restaurant_db::RestaurantDB;

pub mod fetch_all_restaurants;

#[derive(Debug, Serialize)]
pub struct ResponseData {
    data: Vec<RestaurantDB>
}
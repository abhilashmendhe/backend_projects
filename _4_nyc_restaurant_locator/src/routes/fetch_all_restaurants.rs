use axum::extract::Query;
/*
SELECT COUNT(*)
FROM nyc_restaurants
WHERE ST_DWithin(
    location::geography,
    ST_SetSRID(ST_Point(40.72979758784154,-73.9982958555126), 4326)::geography,
    500  -- 50 km
);
*/
use axum::Json;
use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;
use tracing::error;

use crate::routes::ResponseData;
use crate::{models::restaurant_db::RestaurantDB, utils::errors::{AppError, WebError}};

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub radius: Option<i32>
}

pub async fn fetch_all_restaurants(
    State(conn): State<PgPool>,
    Query(q): Query<QueryParams>
) -> Result<(StatusCode, Json<ResponseData>), WebError> {

    if q.lat.is_none() || q.long.is_none() || q.radius.is_none() {
        return Err(WebError::AppError(
            AppError::new(StatusCode::BAD_REQUEST, "Missing coordinates and radius")
        ));
    } 

    let lat = q.lat.unwrap();
    let long = q.long.unwrap();
    let radius = q.radius.unwrap();

    let result = sqlx::query(r#"
    SELECT id,url,name,rating,rating_count,detailed_ratings,price_category,address,json_build_object(
    'srid', ST_SRID(location),
    'lat', ST_Y(location),
    'lon', ST_X(location)
) AS location, zipcode FROM nyc_restaurants 
 WHERE ST_DWithin(
    location::geography,
    ST_SetSRID(ST_Point($1,$2), 4326)::geography,
    $3  
)"#)
.bind(lat)
.bind(long)
.bind(radius)
    .fetch_all(&conn)
    .await
    .map_err(|err| {
        error!("Error fetching all the NYC restaurants: {:?}",err);
        WebError::AppError(
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Problem fetching all the restaurants in NYC. Please try again later!"
            )
        )
    })?;

    let restaurants = result
                    .iter()
                    .map(|row| RestaurantDB {
                        id: row.get("id"),
                        url: row.get("url"),
                        name: row.get("name"),
                        rating: row.get("rating"),
                        rating_count: row.get("rating_count"),
                        detailed_ratings: row.get("detailed_ratings"),
                        price_category: row.get("price_category"),
                        address: row.get("address"),
                        location: row.get("location"),
                        zipcode: row.get("zipcode"),
                    })
                    .collect::<Vec<_>>();
    Ok(
        (StatusCode::OK, Json(ResponseData {data: restaurants}))
    )
}

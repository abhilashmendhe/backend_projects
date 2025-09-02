use axum::Json;
use axum::{extract::State, http::StatusCode};
use sqlx::PgPool;
use sqlx::Row;
use tracing::error;

use crate::routes::ResponseData;
use crate::{models::restaurant_db::RestaurantDB, utils::errors::{AppError, WebError}};

pub async fn fetch_all_restaurants(
    State(conn): State<PgPool>
) -> Result<(StatusCode, Json<ResponseData>), WebError> {

    let result = sqlx::query(r#"
    select id,url,name,rating,rating_count,detailed_ratings,price_category,address,json_build_object(
    'srid', ST_SRID(location),
    'lat', ST_Y(location),
    'lon', ST_X(location)
) AS location, zipcode from nyc_restaurants
    "#)
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

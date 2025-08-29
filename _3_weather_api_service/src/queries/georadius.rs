use redis::{aio::MultiplexedConnection, geo::{RadiusOptions, RadiusOrder, RadiusSearchResult}, AsyncCommands};

use crate::utils::errors::WeatherServiceErr;

pub async fn geo_radius(
    mut conn: MultiplexedConnection,
    lat: f64,
    long: f64
) -> Result<Vec<RadiusSearchResult>, WeatherServiceErr> {
    let radius_opts = RadiusOptions::default().with_dist().order(RadiusOrder::Asc);
    let result = conn
        .geo_radius::<&str, Vec<RadiusSearchResult>>(
            "location", 
            lat, 
            long, 
            3.0, 
            redis::geo::Unit::Kilometers,
            radius_opts
        ).await?;
    Ok(result)
}
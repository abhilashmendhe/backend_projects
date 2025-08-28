use std::fmt::Debug;

use axum::{extract::{FromRequest, Query}, response::{IntoResponse, Response}, Json};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer};
use serde_json::json;
use tracing::error;

#[derive(Debug, Deserialize, Clone)]
pub enum Loc {
    TEXT(String),
    NUM((f64,f64))
}

#[derive(Debug, Deserialize, Clone)]
pub struct QueryParam{
    #[serde(deserialize_with="loc_deserializer")]
    pub location: Loc,
    pub unit: Option<String>
}

fn loc_deserializer<'de,D>(deserializer: D) -> Result<Loc, D::Error>
where 
    D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.len() > 0 {
            let s_points = s.split(",").collect::<Vec<&str>>();
            println!("{:?}",s_points);
            if s_points.len() == 1 {
                let cityname = s_points[0];
                Ok(Loc::TEXT(cityname.to_string()))
            } else if s_points.len() == 2 {
                let lat = s_points[0].parse::<f64>()
                        .map_err(|err|{
                            error!("{:?}",err);
                            de::Error::custom("Failed to parse `latitude`")                         
                        })?;
                let long = s_points[1].parse::<f64>()
                        .map_err(|err|{
                            error!("{:?}",err);
                            de::Error::custom("Failed to parse `longitude`")                         
                        })?;
                if (lat >= -90.0 && lat <= 90.0) && (long >= -180.0 && long <= 180.0) {
                    Ok(Loc::NUM((lat,long)))
                } else {
                    Err(de::Error::custom("`latitue,longitude` values are out of range."))
                }
            } else {    
                Err(de::Error::custom("To many location arguments"))                
            }
        } else {
            Err(de::Error::custom("Please provide `city name` or `lat,long`"))
        }
    }

#[derive(Debug)]
pub struct ValidateLoc<T>(pub T);

impl<S> FromRequest<S> for ValidateLoc<QueryParam>
where 
    // T: DeserializeOwned + Send + Debug,
    S: Send + Sync + Clone
{
    type Rejection = Response;
    async  fn from_request(req:axum::extract::Request,state: &S,) -> Result<Self,Self::Rejection> {
        
        match Query::<QueryParam>::from_request(req, state).await {
            Ok(res) => {
                return Ok(ValidateLoc(res.0));
            },
            Err(err) => {
                error!("{:?}",err);
                let err_msg = err.to_string();
                let body = Json(json!({
                    "error": "Invalid query parameters",
                    "details": err_msg
                }));
                Err((StatusCode::BAD_REQUEST, body).into_response())
            },
        }
    }
}
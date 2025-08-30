use std::sync::Arc;

use axum::{body::Body, extract::{Request, State}, middleware::Next, response::Response};
use reqwest::StatusCode;
use tokio::{sync::Mutex, time::Instant};

use crate::utils::{errors::WebServerErr, rate_limit::RateLimit};

pub async fn rate_limit(
    State(rate_limit): State<Arc<Mutex<RateLimit>>>,
    request: Request<Body>,
    next: Next    
) -> Result<Response, WebServerErr> {
    // println!("i am rate limit");
    let current_time = Instant::now();
    {
        let mut rate_limit_gaurd = rate_limit.lock().await;
        let res = current_time.duration_since(rate_limit_gaurd.time);
        if res.as_secs_f64() > 60.0 {
            rate_limit_gaurd.time = Instant::now();
            rate_limit_gaurd.count = 0;
        } else {
            if rate_limit_gaurd.count > 5 {
                // return too many requests
                // println!("too many requests");
                return Err(
                    WebServerErr::new(
                        StatusCode::TOO_MANY_REQUESTS, 
                        "Too many requests. Please wait for 60s"
                    )
                );
            } else {
                rate_limit_gaurd.count += 1;
            }
        }
    }
    Ok(next.run(request).await)
}
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

/*
    GET /rooms/{room_id}/occupancy?window=1m|5m|1h
*/
#[derive(Debug, Deserialize)]
pub struct WindowQuery {
    pub window: String,
}
pub async fn get_room_occupancy(
    room_id: web::Path<String>,
    window_query: web::Query<WindowQuery>,
) -> impl Responder {
    println!("room id: {}", room_id.into_inner());
    println!("win query: {:?}", window_query.into_inner());
    HttpResponse::Ok()
}

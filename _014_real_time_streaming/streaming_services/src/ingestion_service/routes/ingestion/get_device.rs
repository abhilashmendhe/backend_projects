use actix_web::{HttpResponse, Responder, web::Path};

/*
    GET /devices/{device_id}/health
*/
pub async fn get_device_health(device_id: Path<String>) -> impl Responder {
    println!("device_id {}", device_id.into_inner());
    HttpResponse::Ok()
}

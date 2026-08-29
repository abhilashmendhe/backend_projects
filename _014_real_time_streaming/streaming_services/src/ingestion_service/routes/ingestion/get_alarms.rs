use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

/*
    GET /alarms?since=<ts>
*/

#[derive(Debug, Deserialize)]
pub struct AlarmQuery {
    pub since: String,
}
pub async fn get_alarms(alarm_query: web::Query<AlarmQuery>) -> impl Responder {
    println!("{:?}", alarm_query.into_inner());
    HttpResponse::Ok()
}

use actix_web::{HttpResponse, http::StatusCode, web};
use serde::Serialize;

use crate::{
    models::payload_req::PayloadReq,
    utils::{
        app_state::AppState,
        errors::{AppError, QueueServerErr},
    },
};

#[derive(Debug, Serialize)]
pub struct PayloadResp {
    job_type: String,
    message: String,
}

pub async fn process_payload_request(
    payload_req: web::Json<PayloadReq>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, QueueServerErr> {
    tracing::info!("->> POST /v1/queue-server/payload-req ");
    let payload_req = payload_req.into_inner();
    let tx = app_state.tx();
    tx.send(payload_req.clone()).await.map_err(|err| {
        tracing::error!("Error recving payload: {:?}", err);
        QueueServerErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to process request",
        ))
    })?;

    let payload_resp = PayloadResp {
        job_type: payload_req.job_type(),
        message: "processing".to_string(),
    };
    Ok(HttpResponse::Ok().json(payload_resp))
}

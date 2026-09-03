use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode, web};
use chrono::Utc;
use prost::Message;
use serde::Serialize;
use tonic::Request;

use crate::{
    models::events_model::Event,
    stream_service::{Event as GrpcEvent, PublishRequest},
    utils::{
        app_state::AppState,
        errors::{AppError, IngestionServiceErr},
    },
};

pub async fn req_post_events(
    req: HttpRequest,
    req_event: web::Json<Event>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder, IngestionServiceErr> {
    // 1. trace
    tracing::info!(
        method = %req.method(),
        path = %req.path(),
        device_id = %req_event.device_id,
        room_id = %req_event.room_id,
        seq = req_event.seq,
        "received event"
    );

    // 2. create grpc payload
    let ts = prost_types::Timestamp {
        seconds: req_event.ts.timestamp(),
        nanos: req_event.ts.timestamp_subsec_nanos() as i32,
    };
    let event_type = req_event.event_type.clone() as i32;
    let state = req_event.state.clone().map(|a| a.clone() as i32);
    let grpc_event = GrpcEvent {
        device_id: req_event.device_id.clone(),
        room_id: req_event.room_id.clone(),
        event_type: event_type,
        ts: Some(ts),
        seq: req_event.seq as u64,
        in_room: req_event.in_room,
        magnitude: req_event.magnitude,
        state,
        confidence: req_event.confidence,
        rssi: req_event.rssi,
    };

    // 3. encode the grpc payload
    let mut payload = vec![];
    grpc_event.encode(&mut payload).map_err(|err| {
        tracing::error!("{:?}", err);
        return IngestionServiceErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to encode grpc payload"),
        ));
    })?;

    // 4. create request grpc struct
    let publish_request = PublishRequest {
        message_id: uuid::Uuid::new_v4().to_string(),
        payload,
        timestamp: Utc::now().timestamp_micros(),
    };

    // 5. Make grpc request
    let grpc_request = Request::new(publish_request);
    let mut client = app_data.grpc_client();
    let grpc_resp = client.publish(grpc_request).await.map_err(|err| {
        tracing::error!("{:?}", err);
        return IngestionServiceErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to get response from grpc stream server!"),
        ));
    })?;

    // 6. Check grpc response
    let response = grpc_resp.into_inner();

    // 7. Send back response to data generator
    #[derive(Debug, Serialize)]
    struct EventResponse {
        ok: bool,
    }
    Ok(HttpResponse::Accepted().json(EventResponse {
        ok: response.accepted,
    }))
}

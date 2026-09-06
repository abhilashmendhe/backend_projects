use tokio_stream::StreamExt;
use tonic::Status;
use tracing::{error, info};

use crate::{stream_service::ConsumerRequest, utils::app_data::ConsumerAppState};

pub async fn start_server_streaming(
    mut consumer_app_state: ConsumerAppState,
) -> Result<(), Status> {
    let client = consumer_app_state.grpc_client();

    let response = client.consumer(ConsumerRequest {}).await?;

    let mut resp_stream = response.into_inner();

    while let Some(recv) = resp_stream.next().await {
        match recv {
            Ok(req_payload) => {
                info!("{:?}", req_payload);
            }
            Err(err) => {
                error!("{:?}", err);
            }
        }
    }

    Ok(())
}

use prost::Message;
use tonic::{Request, Response, Status, async_trait};

use crate::{
    create_server::stream_server::{
        Event, PublishRequest, PublishResponse, message_stream_server::MessageStreamServer,
    },
    utils::errors::StreamServerErr,
};

pub mod stream_server {
    tonic::include_proto!("stream_service");
}

#[derive(Debug)]
pub struct StreamServer {}

//  for StreamServer {
// }

#[async_trait]
impl stream_server::message_stream_server::MessageStream for StreamServer {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        // println!("{:?}", request);
        let publish_request = request.into_inner();
        let g_event = Event::decode(publish_request.payload.as_ref()).map_err(|err| {
            tracing::error!("{:?}", err);
            return Status::new(tonic::Code::Internal, "Failed to decode event payload!");
        })?;
        let publish_response = PublishResponse {
            message_id: "123".to_string(),
            accepted: true,
            message: "yo".to_string(),
        };
        Ok(Response::new(publish_response))
    }
}

pub fn stream_server_service() -> MessageStreamServer<StreamServer> {
    MessageStreamServer::new(StreamServer {})
}

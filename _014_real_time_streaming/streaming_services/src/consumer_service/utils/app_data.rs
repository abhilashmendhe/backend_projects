use tonic::transport::Channel;

use crate::stream_service::message_stream_client::MessageStreamClient;

#[derive(Debug)]
#[allow(unused)]
pub struct ConsumerAppState {
    grpc_client: MessageStreamClient<Channel>,
}

impl ConsumerAppState {
    pub fn new(grpc_client: MessageStreamClient<Channel>) -> Self {
        Self { grpc_client }
    }
    pub fn grpc_client(&mut self) -> &mut MessageStreamClient<Channel> {
        &mut self.grpc_client
    }
}

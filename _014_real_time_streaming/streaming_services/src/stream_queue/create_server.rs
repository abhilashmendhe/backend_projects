use std::{pin::Pin, time::Duration};

use tokio::sync::{
    Mutex,
    mpsc::{Receiver, Sender},
};
use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, async_trait};
use tracing::error;

use crate::{
    create_server::stream_server::{
        AckRequest, AckResponse, ConsumerRequest, ConsumerResponse, PublishRequest,
        PublishResponse, message_stream_server::MessageStreamServer,
    },
    utils::app_data::StreamAppData,
};

pub mod stream_server {
    tonic::include_proto!("stream_service");
}

#[derive(Debug)]
pub struct StreamServer {
    app_data: Mutex<StreamAppData>,
    tx: Sender<(u64, PublishRequest)>,
    rx: Mutex<Receiver<(u64, PublishRequest)>>,
    // consumers: Vec
}

#[async_trait]
impl stream_server::message_stream_server::MessageStream for StreamServer {
    type ConsumerStream = Pin<Box<dyn Stream<Item = Result<ConsumerResponse, Status>> + Send>>;

    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        // 1. Get app-data, and tx
        let tx = self.tx.clone();
        // let app_data = &mut self.app_data.lock().await;
        // let w_logger = &mut app_data.logger;
        let w_logger = { &mut self.app_data.lock().await.logger };

        // 2. read publish request
        let publish_request = request.into_inner();
        if tx.capacity() == 0 {
            println!("Please halt..");
        }
        // 3. Append publish request to wal-log
        let (start_offset, _end_offset) = match w_logger.write_log(
            crate::utils::logger::GotRequest::PublishRequest(&publish_request),
        ) {
            Ok((start_offset, _end_offset)) => (start_offset, _end_offset),
            Err(_) => (0, 0),
        };

        // 4. push to channel
        match tx.try_send((start_offset, publish_request.clone())) {
            Ok(_) => {
                println!("Data succefully sent");
            }
            Err(err) => {
                error!("{:?}", err);
                println!("Failed to send.. Buffer full");
            }
        };
        println!("available: {}", tx.capacity());
        println!("max: {}", tx.max_capacity());
        // while let Some(v) = rx.recv().await {
        //     println!("{:?}",v);
        // }
        // println!("{}", sender.max_capacity());
        // let _g_event = Event::decode(publish_request.payload.as_ref()).map_err(|err| {
        //     tracing::error!("{:?}", err);
        //     return Status::new(tonic::Code::Internal, "Failed to decode event payload!");
        // })?;
        let publish_response = PublishResponse {
            message_id: publish_request.message_id,
            accepted: true,
            message: "Enqueued".to_string(),
        };
        Ok(Response::new(publish_response))
    }

    async fn consumer(
        &self,
        req: Request<ConsumerRequest>,
    ) -> Result<Response<Self::ConsumerStream>, Status> {
        println!("\tclient connected from: {:?}", req.remote_addr());
        // 1. get the rx
        let (offset, consumer_resp) = {
            let mut mut_rx = self.rx.lock().await;
            if let Some(consumer_resp) = mut_rx.recv().await {
                (consumer_resp.0, consumer_resp.1)
            } else {
                return Err(Status::new(
                    tonic::Code::Unavailable,
                    "No data available to process!",
                ));
            }
        };

        // 2. create infinite stream
        let repeat = std::iter::repeat(ConsumerResponse {
            offset,
            message_id: consumer_resp.message_id,
            payload: consumer_resp.payload,
            timestamp: consumer_resp.timestamp,
        });

        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // 3. Create channel
        let (stx, srx) = tokio::sync::mpsc::channel(128);

        // 4. spawn
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match stx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be sent to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });
        // println!("{}", );
        // Err(Status::new(
        //     tonic::Code::Unimplemented,
        //     "Server side streaming not implemented!",
        // ))
        let output_stream = ReceiverStream::new(srx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ConsumerStream
        ))
    }

    async fn ack(&self, request: Request<AckRequest>) -> Result<Response<AckResponse>, Status> {
        // 1. Get ack request
        let ack_request = request.into_inner();

        // 2. aof to wal-logs
        // let app_data = &mut self.app_data.lock().await;
        // let w_logger = &mut app_data.logger;
        let w_logger = { &mut self.app_data.lock().await.logger };
        let _ = w_logger.write_log(crate::utils::logger::GotRequest::AckRequest(&ack_request));

        // 3. Send response back
        Ok(Response::new(AckResponse { accepted: true }))
    }
}

pub fn stream_server_service(
    app_data: Mutex<StreamAppData>,
    tx: Sender<(u64, PublishRequest)>,
    rx: Mutex<Receiver<(u64, PublishRequest)>>,
) -> MessageStreamServer<StreamServer> {
    MessageStreamServer::new(StreamServer { app_data, tx, rx })
}

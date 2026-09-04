use std::pin::Pin;

use tokio::sync::{
    Mutex,
    mpsc::{Receiver, Sender},
};
use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming, async_trait};
use tracing::error;

use crate::{
    create_server::stream_server::{
        ConsumerRequest, ConsumerResponse, PublishRequest, PublishResponse,
        consumer_request::Request as CRequest, message_stream_server::MessageStreamServer,
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
}

#[async_trait]
impl stream_server::message_stream_server::MessageStream for StreamServer {
    // type ConsumerStream = Pin<Box<dyn Stream<Item = Result<ConsumerResponse, Status>>+Send>>;
    type ConsumerStream = Pin<Box<dyn Stream<Item = Result<ConsumerResponse, Status>> + Send>>;

    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        // 1. Get app-data, and tx
        let tx = self.tx.clone();
        let app_data = &mut self.app_data.lock().await;
        let w_logger = &mut app_data.logger;

        // 2. read publish request
        let publish_request = request.into_inner();
        if tx.capacity() == 0 {
            println!("Please halt..");
        }
        // 3. Append publish request to wal-log
        let (start_offset, _end_offset) = match w_logger.write_log(&publish_request) {
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
        req: Request<Streaming<ConsumerRequest>>,
    ) -> Result<Response<Self::ConsumerStream>, Status> {
        let rx = &mut self.rx.lock().await;
        let (offset, publish_request) = match rx.recv().await {
            Some(event) => event,
            None => {
                return Err(Status::new(
                    tonic::Code::Unavailable,
                    "Events unavailable to process!".to_string(),
                ));
            }
        };
        println!("{} -> {:?}", offset, publish_request);
        let mut in_stream = req.into_inner();
        let (stream_tx, stream_rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(consumer_req) => {
                        if let Some(cr) = consumer_req.request {
                            // stream_tx.send(value);
                            match cr {
                                CRequest::Ack(_ack_value) => {
                                    // please append to wal as status:Ack
                                }
                                CRequest::Subscribe(_) => {
                                    // Send response
                                    let _ = stream_tx
                                        .send(Ok(ConsumerResponse {
                                            offset,
                                            message_id: publish_request.message_id.clone(),
                                            payload: publish_request.payload.clone(),
                                            timestamp: publish_request.timestamp,
                                        }))
                                        .await;
                                }
                            };
                        }
                    }
                    Err(err) => {
                        error!("{:?}", err);
                        // if let Some(io_err) = match_for_io_error(&err)
                        //     && io_err.kind() == ErrorKind::BrokenPipe
                        // {
                        //     // here you can handle special case when client
                        //     // disconnected in unexpected way
                        //     eprintln!("\tclient disconnected: broken pipe");
                        //     break;
                        // }

                        // match tx.send(Err(err)).await {
                        //     Ok(_) => (),
                        //     Err(_err) => break, // response was dropped
                        // }
                    }
                }
            }
        });
        // let consumer_resp = ConsumerResponse { offset: todo!(), message_id: todo!(), payload: todo!(), timestamp: todo!() };

        let out_stream = ReceiverStream::new(stream_rx);
        Ok(Response::new(Box::pin(out_stream) as Self::ConsumerStream))
    }
}

pub fn stream_server_service(
    app_data: Mutex<StreamAppData>,
    tx: Sender<(u64, PublishRequest)>,
    rx: Mutex<Receiver<(u64, PublishRequest)>>,
) -> MessageStreamServer<StreamServer> {
    MessageStreamServer::new(StreamServer { app_data, tx, rx })
}

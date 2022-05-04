pub mod pb {
    tonic::include_proto!("test_bidi_stream");
}

use futures::Stream;
use std::cmp::{max, min};
use std::{net::ToSocketAddrs, pin::Pin};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use pb::{Req, Res};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Res, Status>> + Send>>;

pub struct RpcBidiStreamServer {}

#[tonic::async_trait]
impl pb::rpc_bidi_stream_server::RpcBidiStream for RpcBidiStreamServer {
    type GoStream = ResponseStream;

    async fn go(&self, req: Request<Streaming<Req>>) -> Result<Response<Self::GoStream>, Status> {
        println!("RpcBidiStreamServer::starting");
        let mut in_stream = req.into_inner();
        let (tx, rx) = mpsc::channel(4096);

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => tx
                        .send(Ok(Res {
                            id: v.id,
                            r: v.s
                                .get(
                                    max(v.i, 0) as usize
                                        ..min(max(v.i + v.c, 0) as usize, v.s.len()),
                                )
                                .unwrap_or("")
                                .repeat(max(1, v.n) as usize),
                        }))
                        .await
                        .expect("working rx"),
                    Err(err) => match tx.send(Err(err)).await {
                        Ok(_) => (),
                        Err(_err) => break,
                    },
                }
            }
            println!("\tstream ended");
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(out_stream) as Self::GoStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = RpcBidiStreamServer {};
    Server::builder()
        .add_service(pb::rpc_bidi_stream_server::RpcBidiStreamServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}

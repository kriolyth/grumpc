use grumpc::{Empty, Item, StatusReply, GrumpyReply};
use grumpc::grumpy_server::{Grumpy, GrumpyServer};
use tonic::{async_trait, Request, Response};

struct Grumper;

#[async_trait]
impl Grumpy for Grumper {
    async fn status(&self, _request: Request<Empty>)
    -> Result<Response<StatusReply>, tonic::Status> {
        Ok(tonic::Response::new(StatusReply {success: true}))
    }
    async fn good_enough(&self, _request: Request<Item>) ->
        Result<Response<GrumpyReply>, tonic::Status> {
        Err(tonic::Status::unimplemented("Method not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, server!");

    let addr = std::net::SocketAddr::from(([0,0,0,0], 3002));
    let service = tonic::transport::Server::builder()
        .add_service(GrumpyServer::new(Grumper {}))
        .serve(addr);
    service.await?;

    Ok(())
}

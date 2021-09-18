use grumpc::grumpy_server::{Grumpy, GrumpyServer};
use grumpc::{Empty, GrumpyReply, Item, StatusReply};
use tonic::{async_trait, Request, Response};

struct Grumper;

impl Grumper {
    fn is_item_good(item: &Item) -> bool {
        item.mood == "grumpy" && item.contents_sentiment == "disappointment"
    }
}

#[async_trait]
impl Grumpy for Grumper {
    async fn status(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<StatusReply>, tonic::Status> {
        Ok(Response::new(StatusReply { success: true }))
    }
    async fn good_enough(
        &self,
        request: Request<Item>,
    ) -> Result<Response<GrumpyReply>, tonic::Status> {
        Ok(Response::new(GrumpyReply {
            good_enough: Grumper::is_item_good(&request.into_inner()),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, server!");

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3002));
    let service = tonic::transport::Server::builder()
        .add_service(GrumpyServer::new(Grumper {}))
        .serve(addr);
    service.await?;

    Ok(())
}

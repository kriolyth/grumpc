use grumpc::grumpy_server::{Grumpy, GrumpyServer};
use grumpc::{Empty, GrumpyReply, Item, StatusReply};
use tonic::{async_trait, Request, Response};

struct Grumper;

impl Grumper {
    // make a decision based on information in fixed fields
    fn is_item_good_partial(item: &Item) -> bool {
        item.mood == "grumpy" && item.contents_sentiment == "disappointment"
    }

    // make a decision based on json contents
    fn is_item_good(item: &Item) -> bool {
        let value: serde_json::Value =
            serde_json::from_str(&item.json_encoded_props).unwrap();

        value["mood"] == "grumpy" && value["sentiment"] == "disappointment"
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
    async fn good_enough_partial(
        &self,
        request: Request<Item>,
    ) -> Result<Response<GrumpyReply>, tonic::Status> {
        Ok(Response::new(GrumpyReply {
            good_enough: Grumper::is_item_good_partial(&request.into_inner()),
        }))
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
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3002));
    let service = tonic::transport::Server::builder()
        .add_service(GrumpyServer::new(Grumper {}))
        .serve(addr.clone());
    println!("Server listening at {}", addr);

    service.await?;

    Ok(())
}

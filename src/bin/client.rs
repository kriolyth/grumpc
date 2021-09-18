use grumpc::{Empty};
use grumpc::grumpy_client::{GrumpyClient};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, client!");

    let mut client = GrumpyClient::connect("http://localhost:3002").await?;
    let res = client.status(Empty {}).await?.into_inner();
    println!("Server status: {}", res.success);

    Ok(())
}

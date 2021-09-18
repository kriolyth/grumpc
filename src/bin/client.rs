use grumpc::grumpy_client::GrumpyClient;
use grumpc::{Empty, Item};

use chrono::prelude::*;

fn make_item() -> Item {
    Item {
        mood: "grumpy".to_owned(),
        contents_sentiment: "disappointment".to_owned(),
        full_text: "no".to_owned(),
    }
}

macro_rules! measure {
    ($f: expr) => {{
        let start_time = Utc::now();
        for _ in 0..1000 {
            $f;
        }
        let duration = Utc::now() - start_time;
        duration.num_milliseconds() as f64 / 1000.
    }};
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, client!");

    let mut client = GrumpyClient::connect("http://localhost:3002").await?;
    let res = client.status(Empty {}).await?.into_inner();
    println!("Server status: {}", res.success);

    // let res = client.good_enough(make_item()).await?.into_inner();
    // println!("Good enough: {}", res.good_enough);
    let elapsed = measure!(client.good_enough(make_item()).await?);
    println!(
        "Elapsed {} s, throughput: {:.2} req/s",
        elapsed,
        1000. / elapsed
    );

    Ok(())
}

use grumpc::grumpy_client::GrumpyClient;
use grumpc::{Empty, Item};

use chrono::prelude::*;
use std::iter::Iterator;

struct ItemGen {
    mood_iter: Box<dyn Iterator<Item = (&'static str, &'static str)> + Sync>,
}
impl ItemGen {
    fn new() -> Self {
        let mood_iter = ["grumpy", "happy", "sleepy", "moody"].iter().cycle();
        let sent_iter = ["disappointment", "neutral", "surprized"].iter().cycle();
        ItemGen {
            mood_iter: Box::new(mood_iter.zip(sent_iter).map(|(&a, &b)| (a, b))),
        }
    }
}
impl Iterator for ItemGen {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let (mood, sent) = self.mood_iter.next().unwrap();
        Some(Item {
            mood: mood.to_owned(),
            contents_sentiment: sent.to_owned(),
            full_text: format!("Some explanation of {} compared to {}", mood, sent),
        })
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
    let mut client = GrumpyClient::connect("http://localhost:3002").await?;
    let res = client.status(Empty {}).await?.into_inner();
    println!("Server status: {}", res.success);

    let mut item_generator = ItemGen::new();
    let elapsed = measure!(client.good_enough(item_generator.next().unwrap()).await?);
    println!(
        "Elapsed {:.3} s, throughput: {:.2} req/s",
        elapsed,
        1000. / elapsed
    );

    Ok(())
}

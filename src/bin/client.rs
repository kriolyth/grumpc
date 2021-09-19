use grumpc::grumpy_client::GrumpyClient;
use grumpc::{Empty, Item};

use chrono::prelude::*;
use std::iter::Iterator;

use serde_json::json;

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
        let js = json!({
            "mood": mood.to_string(),
            "sentiment": sent.to_string(),
            "additional_properties": [
                {"id": 1, "prop": "source", "value": "twitter"},
            ],
            "description": "Grumpy response",
            "full_text": format!("Some explanation of {} compared to {}", mood, sent),
        });
        Some(Item {
            mood: mood.to_owned(),
            contents_sentiment: sent.to_owned(),
            json_encoded_props: serde_json::to_string(&js).unwrap(),
        })
    }
}

macro_rules! measure {
    ($name: literal, $f: expr) => {{
        let start_time = Utc::now();
        for _ in 0..1000 {
            $f;
        }
        let duration = Utc::now() - start_time;
        let elapsed = duration.num_milliseconds() as f64 / 1000.;
        println!(
            "{}: {:.3} s, throughput: {:.2} req/s",
            $name, elapsed,
            1000. / elapsed
        );
    }};
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GrumpyClient::connect("http://localhost:3002").await?;
    let res = client.status(Empty {}).await?.into_inner();
    println!("Server status: {}", res.success);

    let mut item_generator = ItemGen::new();
    measure!("good_enough_partial", 
        client
            .good_enough_partial(item_generator.next().unwrap())
            .await?
    );
    measure!("good_enough", 
        client
            .good_enough(item_generator.next().unwrap())
            .await?
    );
    

    Ok(())
}

use std::env;

use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Channel {
    id: String,
}

#[derive(Serialize, Deserialize)]
struct Channels {
    ok: bool,
    channels: Vec<Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let token = args.get(1).unwrap();
    let client = reqwest::Client::new();
    let resp = client
        .get(
            "https://slack.com/api/conversations.list?types=public_channel,private_channel,mpim,im",
        )
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;
    if resp.status() == 200 {
        let json = resp.json::<Channels>().await?;
        if json.ok {
            json.channels.into_iter().for_each(|c| println!("{}", c.id));
        }
    }
    Ok(())
}

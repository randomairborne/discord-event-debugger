use futures_util::StreamExt;
use tokio::task::JoinSet;
use twilight_gateway::{CloseFrame, Intents, ShardId};

#[tokio::main]
async fn main() {
    let token =
        std::env::var("DISCORD_TOKEN").expect("failed to find DISCORD_TOKEN in environment");
    let intents: u64 = std::env::args()
        .nth(1)
        .expect("Expected encoded gateway intents as first argument: https://discord.com/developers/docs/topics/gateway#gateway-intents")
        .parse()
        .expect("Expected encoded gateway intents as first argument: https://discord.com/developers/docs/topics/gateway#gateway-intents");
    let mut seq = 0;
    let mut shard =
        twilight_gateway::Shard::new(ShardId::ONE, token, Intents::from_bits_truncate(intents));

    let mut js = JoinSet::new();

    std::fs::remove_dir_all("events").ok();
    std::fs::create_dir("events").unwrap();
    let ss = shard.sender();

    tokio::spawn(async move {
        vss::shutdown_signal().await;
        eprintln!("Shutting down");
        ss.close(CloseFrame::NORMAL)
    });

    while let Some(Ok(message)) = shard.next().await {
        match message {
            twilight_gateway::Message::Close(_) => break,
            twilight_gateway::Message::Text(text) => {
                js.spawn(save_message(text, {
                    seq += 1;
                    seq - 1
                }));
            }
        }
    }

    while js.join_next().await.is_some() {}
    println!("Done, bye!");
}

async fn save_message(text: String, seq: u64) {
    tokio::fs::write(format!("events/{seq}.json"), text)
        .await
        .unwrap();
}

use serenity::all::{ClientBuilder, GatewayIntents};

pub mod handler;
pub mod commands;

use handler::Handler;

#[tokio::main]
async fn main() {
    let token = dotenv::var("TOKEN").unwrap();
    let intents = GatewayIntents::GUILDS;

    let mut client = ClientBuilder::new(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    if let Err(e) = client.start().await {
        println!("Failed to start client: {}", e);
    }
}

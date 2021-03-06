#![allow(non_snake_case)]
mod handler;
use handler::Handler;
use serenity::prelude::*;
use serenity::cache::Cache;

// const TOKEN: &str = "";

#[tokio::main]
async fn main() {
    let mut token = std::fs::read_to_string("SECRET").unwrap();
    token = token.trim().to_string();

    let mut Client = Client::builder(
        token,
        GatewayIntents::all()
    ).event_handler(Handler { cache: Cache::new() }).await.unwrap();

    if let Err(reason) = Client.start().await {
    	eprintln!("Bot failed to start!  Error: {reason}");
    }
    // println!("Hello, world!");
}
#![allow(clippy::single_match, clippy::needless_late_init)]
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::{self, gateway::Ready};
use model::channel::Message;

mod handler;

const PREFIX: &str = "$";
// const TOKEN: &str = "";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		println!("{}", &msg.content);
		if msg.content.starts_with(PREFIX) {
			let mut args = msg.content.split(' ')
				.map(|s| s.to_string())
				.collect::<Vec<String>>();
			let mut command: String = args.remove(0);
			command.remove(0);

			/*
			COMMAND HANDLER
			 */
			let res: Result<Message, _>;
			match command .as_str() {
				"test" => {
					res = msg.channel_id.say(
						ctx.http,
						"test"
					).await;
				},
				_ => {
					res = msg.channel_id.say(
						ctx.http,
						"Invalid command!"
					).await;
				}
			}
			if let Ok(m) = res {
				println!("Sent message {}", m.content);
			} else if let Err(e) = res {
				println!("An error occurred sending message: {}", e);
			}
		}
	}
	async fn ready(&self, _: Context, ready: Ready) {
		println!("Connected to Discord as {}", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
	let mut token = std::fs::read_to_string("SECRET").unwrap();
	token = token.trim().to_string();

	let mut Client = Client::builder(
		token,
		GatewayIntents::all()
	).event_handler(Handler).await.unwrap();

	if let Err(reason) = Client.start().await {
		eprintln!("Bot failed to start!  Error: {reason}");
	}
	// println!("Hello, world!");
}

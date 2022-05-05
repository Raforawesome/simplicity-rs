#![allow(dead_code)]
mod commands;

use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::{self, gateway::Ready};
use model::channel::Message;

use commands::COMMANDS;

const PREFIX: &str = "$";

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

			if let Some(f) =  COMMANDS.get(command.as_str()) {
				f(&ctx, &msg, &args);
			}
		}
	}
	async fn ready(&self, _: Context, ready: Ready) {
		println!("Connected to Discord as {}", ready.user.name);
	}
}
#![allow(dead_code)]
mod commands;

use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::{self, gateway::Ready};
use model::channel::Message;
use serenity::cache::Cache;

use commands::COMMANDS;

const PREFIX: &str = "$";

pub struct Handler {
	pub cache: Cache
}

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content.starts_with(PREFIX) {
			let mut args = msg.content.split(' ')
				.map(|s| s.to_string())
				.collect::<Vec<String>>();
			let mut command: String = args.remove(0);
			command.remove(0);

			if let Some(f) =  COMMANDS.get(command.as_str()) {
				let _ = f(ctx, msg, args).await;
			} else {
				let cmd_names = COMMANDS.keys().copied().collect::<Vec<&str>>();
				let mut most_similar: (&str, f32) = ("", 0_f32);
				for name in cmd_names {
					let (_, score) = most_similar;
					let new_score = similarity(&command, name);
					if new_score > score {
						most_similar = (name, new_score);
					}
				}

				let (similar, score) = most_similar;
				if score >= 50_f32 {
					let _ = msg.channel_id.send_message(
						ctx.http,
						|m| {
							m.embed(|e| {
								e.description(format!("Invalid command!  Did you mean `${}`?", similar))
									.color((255, 0, 0))
							})
						}
					).await;
				}
			}
		}
	}
	async fn ready(&self, _: Context, ready: Ready) {
		println!("Connected to Discord as {}", ready.user.name);
	}
}

fn similarity(s1: &str, s2: &str) -> f32 {
	let mut similar: f32 = 0_f32;
	let total: f32 = (if s2.len() > s1.len() {s1.len()} else {s2.len()}) as f32;

	if s2.len() > s1.len() {
		for (i, char) in s1.chars().enumerate() {
			let e = &s2[i..i+1];
			if char.to_string().as_str() == e {
				similar += 1_f32;
			}
		}
	} else {
		for (i, char) in s2.chars().enumerate() {
			let e = &s1[i..i+1];
			if char.to_string().as_str() == e {
				similar += 1_f32;
			}
		}
	}
	(similar / total) * 100_f32
}
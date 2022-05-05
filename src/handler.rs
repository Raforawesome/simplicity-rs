#![allow(dead_code)]
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::{self, gateway::Ready};
use model::channel::Message;
mod commands;

const PREFIX: &str = "$";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		println!("{:?}", msg);
	}
	async fn ready(&self, _: Context, ready: Ready) {
		println!("Connected to Discord as {}", ready.user.name);
	}
}
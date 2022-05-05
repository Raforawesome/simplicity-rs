#![allow(unused_variables, dead_code)]
use std::future::Future;
use std::task::Poll;
use serenity::futures::TryFutureExt;
use super::prelude::*;
use super::utils;

pub const CMD: Command = Command {
	command: "targettest",
	// aliases: &["testcmd"],
	self_allowed: false,
	execute
};

// type Ret = Box<dyn Future<Output = Result<Message, serenity::Error>> + Send + Sync>;
type Ret = Pin<Box<dyn Future<Output = ()> + Send>>;
pub fn execute(ctx: Context, msg: Message, args: Vec<String>) -> Ret {
	Box::pin(execute_wrap(ctx, msg, args))
}

pub async fn execute_wrap(ctx: Context, msg: Message, args: Vec<String>) {
	let mut me: String = String::new();
	if msg.mentions.is_empty() && args.is_empty() {
		me = "Not enough arguments specified!".to_string();
	}
	let target = tokio::task::spawn(utils::get_targets(
		ctx.clone(),
		msg.mentions.clone(),
		args[0].to_owned(),
		msg.guild_id
	)).await.unwrap();

	if let Some(b) = target {
		let u = *b;
		me = format!("Found user {:?}", u);
	} else {
		me = "No user found.".to_string();
	}

	let res = msg.channel_id.send_message(
		ctx.http,
		|m| {
			m.embed(|e| {
				e.description(me)
			})
		}
	).await;
}
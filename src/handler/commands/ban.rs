#![allow(unused_variables, dead_code)]
use std::future::Future;
use serenity::futures::TryFutureExt;
use crate::handler::commands::utils;
use super::prelude::*;
use super::utils::*;

pub const CMD: Command = Command {
	command: "ban",
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
	let target = get_target_mem(
		&ctx,
		msg.mentions.clone(),
		args[0].clone(),
		msg.guild_id
	).await;
	let author = msg.author.clone();

	if let Some(t) = target {
		let target = *t;
		let reason = format!("By: {} ({})", author.name, author.id.as_u64());
		let res = target.ban_with_reason(&ctx.http, 0, reason).await;
		if res.is_err() {
			let _ = send_embed("Unable to ban target!  Do I have permission?", &msg, &ctx, (255, 0, 0));
		}
	} else {
		let _ = send_embed("Invalid target!", &msg, &ctx, (255, 0, 0));
	}
}
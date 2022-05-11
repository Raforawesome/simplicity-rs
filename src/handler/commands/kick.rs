#![allow(unused_variables, dead_code)]
use std::future::Future;
use serenity::futures::TryFutureExt;
use crate::handler::commands::utils;
use super::prelude::*;
use super::utils::*;

pub const CMD: Command = Command {
	command: "kick",
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
		let reason = format!("By: {} ({})", author.tag(), author.id.as_u64());
		let res = target.kick_with_reason(&ctx.http, reason.as_str()).await;
		if res.is_err() {
			let _ = send_embed("Unable to kick target!  Do I have permission?", &msg, &ctx, (255, 0, 0)).await;
		} else {
			let _ = send_embed(format!("Kicked <@{}>.", target.user.id.as_u64()), &msg, &ctx, (255, 0, 255)).await;
		}
	} else {
		let _ = send_embed("Invalid target!", &msg, &ctx, (255, 0, 0)).await;
	}
}
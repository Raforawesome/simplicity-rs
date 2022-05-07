#![allow(unused_variables, dead_code)]
use std::future::Future;
use serenity::futures::TryFutureExt;
use crate::handler::commands::utils::send_embed;
use super::prelude::*;
use super::utils::get_targets;

pub const CMD: Command = Command {
	command: "avatar",
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
	let target = get_targets(&ctx, msg.mentions.clone(), args[0].clone(), msg.guild_id)
		.await;

	if let Some(usr) = target {
		let user = *usr;
		let _ = msg.channel_id.send_message(
			ctx.http,
			|m| {
				m.embed(|e| {
					e.image(user.avatar_url().unwrap())
				})
			}
		).await;
	} else {
		let _ = send_embed("Invalid target!", &msg, &ctx, (255, 0, 0));
	}
}
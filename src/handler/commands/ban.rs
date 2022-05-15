#![allow(unused_variables, dead_code)]
use std::future::Future;
use serenity::futures::TryFutureExt;
use crate::handler::commands::utils;
use super::prelude::*;
use super::utils::*;
use std::sync::Arc;

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
	if msg.guild_id.is_none() {
		let _ = send_embed("This command can only be used in a server!",
		&msg,
		&ctx,
		(255, 0, 0));
		return;
	}

	let gid = msg.guild_id.unwrap();
	let author_member = gid.member(&ctx.http,
		msg.author.id).await.unwrap();
	if !author_member.permissions(&ctx.cache).unwrap().ban_members() {
		let _ = send_embed("You lack the `BAN_MEMBERS` permission!",
		&msg,
		&ctx,
		(255, 0, 0));
		return;
	}


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
		let res = target.ban_with_reason(&ctx.http, 0, reason).await;
		if res.is_err() {
			let _ = send_embed("Unable to ban target!  Do I have permission?", &msg, &ctx, (255, 0, 0)).await;
		} else {
			let _ = send_embed(format!("Banned <@{}>.", target.user.id.as_u64()), &msg, &ctx, (255, 0, 255)).await;
		}
	} else {
		let _ = send_embed("Invalid target!", &msg, &ctx, (255, 0, 0)).await;
	}
}
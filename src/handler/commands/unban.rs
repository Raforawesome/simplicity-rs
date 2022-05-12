#![allow(unused_variables, dead_code)]
use std::future::Future;
use serenity::futures::TryFutureExt;
use crate::handler::commands::utils::send_embed;
use super::prelude::*;

pub const CMD: Command = Command {
	command: "unban",
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
	if !author_member.permissions.unwrap().ban_members() {
		let _ = send_embed("You lack the `BAN_MEMBERS` permission!",
		&msg,
		&ctx,
		(255, 0, 0));
		return;
	}


	let gid = if let Some(id) = msg.guild_id { id } else {
		let _ = send_embed("This command must be run in a server!", &msg, &ctx, (255, 0, 0)).await;
		return;
	};

	let id = if let Ok(n) = args[0].parse::<u64>() {n} else {
		let _ = send_embed("Provided input is not a valid UserID!", &msg, &ctx, (255, 0, 0)).await;
		return;
	};
	let bans = if let Ok(res) = gid.bans(&ctx.http).await { res }
	else {
		let _ = send_embed("Failed to fetch bans.  Do I have the `Ban Members` permission?", &msg, &ctx, (255, 0, 0)).await;
		return;
	};

	let mut ban = None;
	for b in bans {
		if b.user.id.as_u64() == &(id as u64) {
			ban = Some(b)
		}
	};

	if let Some(b) = ban {
		let _ = gid.unban(&ctx.http, id as u64).await;
		let _ = send_embed(format!("Unbanned <@{}>.", id),
						   &msg, &ctx, (255, 0, 255)).await;
	}
}
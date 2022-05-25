#![allow(unused_variables, dead_code, clippy::needless_late_init)]
use std::future::Future;
use serenity::futures::TryFutureExt;
use super::prelude::*;
use serenity::Error as serr;
use crate::handler::commands::utils::send_embed;

pub const CMD: Command = Command {
	command: "purge",
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
	if !author_member.permissions(&ctx.cache).unwrap().manage_messages() {
		let _ = send_embed("You lack the `MANAGE_MESSAGES` permission!",
		&msg,
		&ctx,
		(255, 0, 0)).await;
		return;
	}


	let c_temp = args[0].parse::<i32>();
	let count: i32;
	if let Ok(n) = c_temp {
		count = n;
	} else {
		let _ = send_embed("Provided input was not a number!", &msg, &ctx, (255, 0, 0)).await;
		return;
	}
	if !(2..=100).contains(&count) {
		println!("found edge");
		let _ = send_embed("Invalid amount.  Amount must be between 2 and 100.", &msg, &ctx, (255, 0, 0)).await;
		return;
	}

	let messages = get_messages_count(&ctx, &msg, count).await.unwrap();
	if messages.is_empty() {
		let _ = send_embed("Empty message count!  Do I have `Read Message History` permissions?",
						   &msg, &ctx, (255, 0, 0));
		return;
	}

	let res = msg.channel_id.delete_messages(
		&ctx.http,
		messages
	).await;
	if res.is_err() {
		let _ = send_embed("Failed to delete messages.  Do I have `Manage Messages` permission?",
						   &msg, &ctx, (255, 0, 0)).await;
	} else {
		let _ = msg.delete(&ctx.http).await;
		let m = send_embed(format!("Deleted {} messages.", count), &msg, &ctx, (255, 0, 255)).await;
		std::thread::sleep(std::time::Duration::from_secs(3));
		let _ = m.delete(&ctx.http).await;
	}
}

pub async fn get_messages_count(ctx: &Context, msg: &Message, count: i32) -> Result<Vec<Message>, serr> {
	let cid = msg.channel_id;
	let messages = cid.messages(&ctx.http, |r| {
		r.before(msg.id).limit(count as u64)
	}).await;
	messages
}
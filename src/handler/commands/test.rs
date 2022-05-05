#![allow(unused_variables, dead_code)]
use std::future::Future;
use std::task::Poll;
use serenity::futures::TryFutureExt;
use super::prelude::*;

pub const CMD: Command = Command {
	command: "test",
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
	let res = msg.channel_id.send_message(
		ctx.http,
		|m| {
			m.embed(|e| {
				e.description("Test.")
			})
		}
	).await;
}
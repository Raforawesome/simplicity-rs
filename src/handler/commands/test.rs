#![allow(unused_variables, dead_code)]
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use pollster::FutureExt;
use serenity::futures::TryFutureExt;
use super::prelude::*;

pub const cmd: Command = Command {
	command: "test",
	// aliases: &["testcmd"],
	self_allowed: false,
	execute
};

type Ret = Box<dyn Future<Output = Result<Message, serenity::Error>> + Send + Sync>;
pub fn execute(ctx: Context, msg: &Message, args: &[String]) -> std::pin::Pin<Ret> {
	let res = msg.channel_id.send_message(
		ctx.http,
		|m| {
			m.embed(|e| {
				e.description("Test.")
			})
		}
	);
	Box::pin(res)
}
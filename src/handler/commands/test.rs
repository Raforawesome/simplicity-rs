#![allow(unused_variables, dead_code)]
use std::future::Future;
use pollster::FutureExt;
use serenity::futures::TryFutureExt;
use super::prelude::*;

use std::pin::Pin;

pub const cmd: Command = Command {
	command: "test",
	aliases: &["testcmd"],
	self_allowed: false,
	execute
};

type Ret = Box<dyn Future<Output = Result<Message, serenity::Error>>>;
pub fn execute(ctx: Context, msg: &Message, args: &[String]) -> Ret {
	println!("Sending message");
	let res = msg.channel_id.say(
		ctx.http,
		"test"
	);
	Box::new(res)
}
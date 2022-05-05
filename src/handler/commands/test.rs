#![allow(unused_imports, unused_variables, dead_code)]

use std::future::Future;
use pollster::FutureExt;
use serenity::futures::TryFutureExt;
use super::prelude::*;

pub const cmd: Command = Command {
	command: "test",
	aliases: &["testcmd"],
	self_allowed: false,
	execute
};

pub fn execute(ctx: &Context, msg: &Message, args: &[String]) {
	println!("Sending message");
	let res = msg.channel_id.say(
		&ctx.http,
		"test"
	).block_on();
	if let Err(e) = res {
		println!("Error in sending message: {}", e);
	} else if let Ok(s) = res {
		println!("Sent {}", s.content);
	}
}
#![allow(unused_imports, unused_variables)]
use super::prelude::*;

const cmd: Command = Command {
	command: "test",
	aliases: &["testcmd"],
	self_allowed: false,
	execute
};

fn execute(ctx: &Context, args: &[&str]) {}
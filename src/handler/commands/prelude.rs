#![allow(unused_variables)]

pub use serenity::prelude::*;
pub use serenity::async_trait;
pub use serenity::model::{self, gateway::Ready};
pub use model::channel::Message;

// pub trait CommandTrait {
// 	fn execute(args: &[&str]) {}
// }

pub struct Command {
	pub command: &'static str,
	pub aliases: &'static [&'static str],
	pub self_allowed: bool,
	pub execute: fn(&Context, &Message, &[String])
}

// impl CommandTrait for Command {}
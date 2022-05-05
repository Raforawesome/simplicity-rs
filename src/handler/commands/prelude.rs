#![allow(unused_variables)]
pub use serenity::prelude::*;
pub use serenity::async_trait;
pub use serenity::model::{self, gateway::Ready};
pub use model::channel::Message;

use std::future::Future;

// pub trait CommandTrait {
// 	fn execute(args: &[&str]) {}
// }

type Ret = Box<dyn Future<Output = Result<Message, serenity::Error>> + Send + Sync>;
pub struct Command {
	pub command: &'static str,
	// pub aliases: &'static [&'static str],
	pub self_allowed: bool,
	pub execute: fn(Context, Message, &[String]) -> std::pin::Pin<Ret>
}

// impl<T> Command<T> {
// 	pub fn new(t: T, command: &'static str, aliases: &'static [&'static str], self_allowed: bool, execute: fn(&Context, &Message, &[String]) -> T) -> Self {
// 		Command {
// 			command,
// 			aliases,
// 			self_allowed,
// 			execute
// 		}
// 	}
// }

// impl CommandTrait for Command {}
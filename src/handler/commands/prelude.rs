#![allow(unused_variables)]
pub use serenity::prelude::*;
pub use serenity::async_trait;
pub use serenity::model::{self, gateway::Ready};
pub use model::channel::Message;
pub use std::pin::Pin;

use std::future::Future;

// pub trait CommandTrait {
// 	fn execute(args: &[&str]) {}
// }

// type Ret = Box<dyn Future<Output = Result<Message, serenity::Error>> + Send + Sync>;
type Ret = Pin<Box<dyn Future<Output = ()> + Send>>;
pub struct Command {
	pub command: &'static str,
	// pub aliases: &'static [&'static str],
	pub self_allowed: bool,
	pub execute: fn(Context, Message, Vec<String>) -> Ret
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
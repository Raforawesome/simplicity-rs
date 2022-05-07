#![allow(unused_variables, dead_code)]
use std::future::Future;
use rand::SeedableRng;
use serenity::futures::TryFutureExt;
use super::prelude::*;
use rand::seq::SliceRandom;

pub const CMD: Command = Command {
	command: "coinflip",
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
	let mut responses: Vec<String> = vec!["Heads.".to_string(), "Tails.".to_string()];
	let ux_time = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
	let mut rng = rand::rngs::StdRng::seed_from_u64(ux_time.as_secs());

	let slice: &mut [String] = &mut responses;
	slice.shuffle(&mut rng);

	let res = msg.channel_id.send_message(
		ctx.http,
		|m| {
			m.embed(|e| {
				e.description(slice[0].to_string())
					.color((0, 255, 0))
			})
		}
	).await;
}
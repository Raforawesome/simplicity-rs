#![allow(unused_variables, dead_code)]
use std::future::Future;
use rand::SeedableRng;
use serenity::futures::TryFutureExt;
use super::prelude::*;
use rand::seq::SliceRandom;

pub const CMD: Command = Command {
	command: "8ball",
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
	let mut responses: Vec<String> = vec!["It is certain.".to_string(), "It is decidedly so.".to_string(), "Without a doubt.".to_string(), "Yes - definitely.".to_string(), "You may rely on it.".to_string(), "As I see it, yes.".to_string(), "Most likely.".to_string(), "Outlook good.".to_string(), "Yes.".to_string(), "Signs point to yes.".to_string(), "Reply hazy, try again.".to_string(), "Ask again later.".to_string(), "Better not tell you now.".to_string(), "Cannot predict now.".to_string(), "Concentrate and ask again.".to_string(), "Don't count on it.".to_string(), "My reply is no.".to_string(), "My sources say no.".to_string(), "Outlook not so good.".to_string(), "Very doubtful.".to_string()];
	let ux_time = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
	let mut rng = rand::rngs::StdRng::seed_from_u64(ux_time.as_secs());

	let slice: &mut [String] = &mut responses;
	slice.shuffle(&mut rng);

	let res = msg.channel_id.send_message(
		ctx.http,
		|m| {
			m.embed(|e| {
				e.description(slice[0].to_string())
			})
		}
	).await;
}
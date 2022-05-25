#![allow(unused_variables, dead_code)]
use std::future::Future;
use serenity::futures::TryFutureExt;
use super::prelude::*;
use super::utils::send_embed;
use serde_json::Value;

pub const CMD: Command = Command {
	command: "minecraftprofile",
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
	if args.is_empty() {
		let _ = send_embed("Please provide a valid username!", &msg, &ctx, (255, 0, 0)).await;
		return;
	}
	
	let username: &String = &args[0];
	let res = reqwest::get(format!(
		"https://api.mojang.com/users/profiles/minecraft/{}",
		username
	)).await;

	if res.is_err() {
		let _ = send_embed("HTTP request failed!", &msg, &ctx, (255, 0, 0)).await;
	}


	let body = res.unwrap().text().await.unwrap();
	let parsed = serde_json::from_str::<Value>(&body).unwrap();

	let (name, id) = (parsed["name"].as_str().unwrap().to_owned(), parsed["name"].as_str().unwrap().to_owned());
	todo!()
}
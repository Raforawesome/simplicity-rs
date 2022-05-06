use std::future::Future;
use serenity::futures::TryFutureExt;
use super::prelude::*;
use super::utils::{send_embed, get_embed};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process;

pub const CMD: Command = Command {
	command: "eval",
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
	let mut args: Vec<String> = args.clone();
	let mode: String = args.remove(0);
	let mut body_temp: String = String::new();

	for arg in args {
		body_temp.push_str(&arg);
		body_temp.push(' ');
	}
	if !body_temp.contains("```") {
		send_embed("ERROR: Put your code in a code block.", &msg, &ctx, (255, 0, 0)).await;
		return;
	}

	let mut body: String = String::new();
	let lines = body_temp.lines()
		.map(|x| x.to_string())
		.collect::<Vec<String>>();
	let max = lines.len() - 1;

	for (i, line) in lines.iter().enumerate() {
		if i == 0 || i == max {
			continue
		} else {
			body.push_str(line);
			body.push('\n');
		}
	}
	////////////////////////
	if mode == "lua" {
		let mut file = fs::File::create("temp.lua").unwrap();
		let _ = file.write_all(body.as_bytes());

		let output = process::Command::new("lua")
			.arg("temp.lua")
			.output().unwrap();
		let stdout_bytes = output.stdout;
		let stdout_string: String = String::from_utf8(stdout_bytes).unwrap();

		let _ = fs::remove_file(PathBuf::from("./temp.lua"));

		send_embed(format!("Lua output:\n```\n{}\n```", stdout_string), &msg, &ctx, (0, 255, 0)).await;

	} else if mode == "python" || mode == "py" {
		let mut file = fs::File::create("temp.py").unwrap();
		let _ = file.write_all(body.as_bytes());

		let output = process::Command::new("python")
			.arg("temp.py")
			.output().unwrap();
		let stdout_bytes = output.stdout;
		let stdout_string: String = String::from_utf8(stdout_bytes).unwrap();

		let _ = fs::remove_file(PathBuf::from("./temp.py"));

		send_embed(format!("Python output:\n```\n{}\n```", stdout_string), &msg, &ctx, (0, 255, 0)).await;

	} else if mode == "js" || mode == "javascript" || mode == "nodejs" {
		let mut file = fs::File::create("temp.js").unwrap();
		let _ = file.write_all(body.as_bytes());

		let output = process::Command::new("node")
			.arg("temp.js")
			.output().unwrap();
		let stdout_bytes = output.stdout;
		let stdout_string: String = String::from_utf8(stdout_bytes).unwrap();

		let _ = fs::remove_file(PathBuf::from("./temp.js"));

		send_embed(format!("Node.JS output:\n```\n{}\n```", stdout_string), &msg, &ctx, (0, 255, 0)).await;

	} else if mode == "c" || mode == "C" {
		let mut file = fs::File::create("temp.c").unwrap();
		let _ = file.write_all(body.as_bytes());


		let mut m = send_embed("`Compiling...`", &msg, &ctx, (255, 255, 0)).await;

		let status = process::Command::new("gcc")
			.arg("temp.c")
			.status().unwrap();

		if !status.success() {
			let _ = m.edit(ctx.http, |m| {
				m.embed(|e| {
					e.description(format!("Compiler exited with error `{}`", status))
						.color((255, 0, 0))
				})
			}).await;
			return;
		}


		let output = process::Command::new("./a.out")
			.output().unwrap();
		let stdout_bytes = output.stdout;
		let stdout_string: String = String::from_utf8(stdout_bytes).unwrap();

		let _ = fs::remove_file(PathBuf::from("./temp.c"));
		let _ = fs::remove_file(PathBuf::from("./a.out"));

		let _ = m.edit(ctx.http, |m| {
			m.embed(|e| {
				e.description(format!("Compiled C output:\n```\n{}\n```", stdout_string))
					.color((0, 255, 0))
			})
		}).await;

	} else if mode == "rust" || mode == "Rust" || mode == "rs" {
		let mut file = fs::File::create("temp.rs").unwrap();
		let _ = file.write_all(body.as_bytes());


		let mut m = send_embed("`Compiling...`", &msg, &ctx, (255, 255, 0)).await;

		let status = process::Command::new("rustc -C opt-level=3")
			.arg("temp.rs")
			.status().unwrap();

		if !status.success() {
			let _ = m.edit(ctx.http, |m| {
				m.embed(|e| {
					e.description(format!("Compiler exited with error `{}`", status))
						.color((255, 0, 0))
				})
			}).await;
			return;
		}


		let output = process::Command::new("./temp")
			.output().unwrap();
		let stdout_bytes = output.stdout;
		let stdout_string: String = String::from_utf8(stdout_bytes).unwrap();

		let _ = fs::remove_file(PathBuf::from("./temp"));
		let _ = fs::remove_file(PathBuf::from("./temp.rs"));

		let _ = m.edit(ctx.http, |m| {
			m.embed(|e| {
				e.description(format!("Compiled Rust output:\n```\n{}\n```", stdout_string))
					.color((0, 255, 0))
			})
		}).await;

	} else {
		send_embed("ERROR: Invalid language!", &msg, &ctx, (255, 0, 0)).await;
	}
}
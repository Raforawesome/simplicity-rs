#![allow(unused_variables, dead_code)]
use std::future::Future;
use serenity::futures::TryFutureExt;
use super::prelude::*;

pub const CMD: Command = Command {
	command: "help",
	// aliases: &["testcmd"],
	self_allowed: false,
	execute
};

const HELP_MAIN: &str = r#"
Help Pages:
Moderation ($help mod)
Utility ($help util)
Fun ($help fun)

Message Colors:
Green - Successful
Red - Error
Yellow - Info
Orange - Warning
Purple - Moderation Command"#;

const HELP_MOD: &str = r#"
$ban <user> - Bans a user from the server.
$unban <id> - Unbans a user given their User ID.
$purge <count> - Deletes <count> amount of messages in this channel.
    - $clear <count> - Alias of $purge.
    - $clr <count> - Alias of $purge."#;

const HELP_UTIL: &str = r#"
$eval <language> ```<code>``` - Evaluates code in <language>.
$evalsupport - Shows which languages $eval currently supports.
$invite - Displays the invite link for this bot.
$targettest - A test command to see if the bot picks up your target."#;

const HELP_FUN: &str = r#"
$avatar <user> - Displays a full scale link to the user's avatar.
    - $av <user> - Alias of $avatar.
    - $pfp <user> - Alias of $avatar.
$coinflip - Flips a coin and displays it's result.
$8ball - Ask the almighty 8-Ball a question."#;

// type Ret = Box<dyn Future<Output = Result<Message, serenity::Error>> + Send + Sync>;
type Ret = Pin<Box<dyn Future<Output = ()> + Send>>;
pub fn execute(ctx: Context, msg: Message, args: Vec<String>) -> Ret {
	Box::pin(execute_wrap(ctx, msg, args))
}
pub async fn execute_wrap(ctx: Context, msg: Message, args: Vec<String>) {
	let res = msg.channel_id.send_message(
		ctx.http,
		|m| {
			m.embed(|e| {
				e.description("Test.")
			})
		}
	).await;
}
use super::prelude::*;
use serenity::builder::{self, CreateEmbed};
use serenity::model::{
	user::User,
	id::GuildId,
	guild::Member,
	channel::Message,
};

pub async fn get_targets(ctx: &Context, mentions: Vec<User>, arg: String, g_id: Option<GuildId>) -> Option<Box<User>> {
	if !mentions.is_empty() {
		return Some(Box::new(mentions[0].clone()));
	}
	let mut r: Option<Box<User>> = None;
	if let Some(gid) = g_id {
		let members_res = gid.members(
			&(ctx.http),
			None,
			None
		).await;
		if let Ok(members) = members_res {
			for member in members {
				if let Some(nick) = member.nick {
					if nick.contains(&arg) {
						r = Some(Box::new(member.user));
						break;
					}
				}
				if member.user.name.contains(&arg) {
					r = Some(Box::new(member.user));
					break;
				}
			}
		} else {
			eprintln!("WARNING: An error occurred in fetching guild members.");
		}
	}
	r
}

pub async fn send_embed<T: ToString>(s: T, msg: &Message, ctx: &Context, color: (u8, u8, u8)) -> Message {
	let m = msg.channel_id.send_message(
		&ctx.http,
		|m| {
			m.embed(|e| {
				e.description(s)
					.color(color)
			})
		}
	).await;
	m.unwrap()
}

pub async fn get_embed<T: ToString>(s: T, msg: &Message, ctx: &Context, color: (u8, u8, u8)) -> CreateEmbed {
	let mut e = builder::CreateEmbed::default();
	e.description(s);
	e.color(color);
	e
}
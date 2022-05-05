use super::prelude::*;
use serenity::model::{
	user::User,
	id::GuildId,
	guild::Member
};

pub async fn get_targets(ctx: Context, mentions: Vec<User>, arg: String, g_id: Option<GuildId>) -> Option<Box<User>> {
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
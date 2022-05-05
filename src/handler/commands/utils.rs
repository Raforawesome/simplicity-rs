use super::prelude::*;
use serenity::model::user::User;

fn get_targets<'a>(mentions: &'a [User], arg: &str) -> Option<&'a User> {
	if !mentions.is_empty() {
		return Some(&mentions[0]);
	}
	let r: Option<&'a User> = None;
	todo!()
}
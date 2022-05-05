use super::prelude::*;
use serenity::model::user::User;

fn get_targets<'a>(mentions: &'a [User], args: &[String]) -> Option<&'a User> {
	if !mentions.is_empty() {
		return Some(&mentions[0]);
	}
	todo!()
}
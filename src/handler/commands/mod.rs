#![allow(dead_code, unused_imports)]
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::{self, gateway::Ready};
use model::channel::Message;

use std::collections::HashMap;
use std::pin::Pin;
use phf::phf_map;

pub mod test;
mod prelude;

type Ret = Box<dyn std::future::Future<Output = Result<Message, serenity::Error>> + Send + Sync>;
type cmdfn = fn(Context, &Message, &[String]) -> std::pin::Pin<Ret>;
pub static COMMANDS: phf::Map<&str, cmdfn> = phf_map! {
    "test" => test::cmd.execute,
};

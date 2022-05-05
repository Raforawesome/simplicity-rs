#![allow(dead_code, unused_imports)]
mod prelude;
mod utils;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::{self, gateway::Ready};
use model::channel::Message;

use std::collections::HashMap;
use std::pin::Pin;
use phf::phf_map;

// COMMANDS \\
mod test;
mod target_test;
//////////////


type Ret = Box<dyn std::future::Future<Output = Result<Message, serenity::Error>> + Send + Sync>;
type cmdfn = fn(Context, Message, &[String]) -> std::pin::Pin<Ret>;
pub static COMMANDS: phf::Map<&str, cmdfn> = phf_map! {
    "test" => test::CMD.execute, "testcmd" => test::CMD.execute,
    "targettest" => target_test::CMD.execute
};

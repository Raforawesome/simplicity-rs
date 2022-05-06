#![allow(dead_code, unused_imports)]
mod prelude;
mod utils;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::{self, gateway::Ready};
use model::channel::Message;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use phf::phf_map;

// COMMANDS \\
mod test;
mod target_test;
mod invite;
// mod eight_ball;
mod eval;
//////////////


// type Ret = Box<dyn std::future::Future<Output = Result<Message, serenity::Error>> + Send + Sync>;
type Ret = Pin<Box<dyn Future<Output = ()> + Send>>;
type cmdfn = fn(Context, Message, Vec<String>) -> Ret;

pub static COMMANDS: phf::Map<&str, cmdfn> = phf_map! {
    "test" => test::CMD.execute, "testcmd" => test::CMD.execute,
    "targettest" => target_test::CMD.execute,
    "invite" => invite::CMD.execute,
    // "8ball" => eight_ball::CMD.execute
    "eval" => eval::CMD.execute,
};

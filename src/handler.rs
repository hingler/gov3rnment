use serenity::{all::{Message, Ready}, async_trait, client::{Context, EventHandler}};

use crate::{args::arg_parser::ArgParser, command::{audio::youtube::YTCommands, demo::pushup::Pushup, hunor::scream::Scream}};
use crate::command::command_repo::CommandRepo;
use crate::command::demo::ping::Ping;

pub struct Handler {
  repo: CommandRepo
}

impl Handler {
  pub fn new() -> Self {
    let mut repo = CommandRepo::new();
    repo.insert(String::from("ping"), Box::new(Ping));
    repo.insert(String::from("pushup"), Box::new(Pushup));
    repo.insert(String::from("scream"), Box::new(Scream));
    repo.insert(String::from("ytdl"), Box::new(YTCommands));
    return Handler { repo };
  }
}



#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    
    let token = "g";

    let parser = ArgParser::new(&token, &msg.content);

    // self is considered immutable :(
    if !msg.is_own(ctx.cache.as_ref()) {
      if let Some(p) = parser {
        println!("message parsed! {}", p.message);
        for i in 0..p.args.len() {
          println!("arg {}: {}", i, p.args[i]); 
  
        }
  
        if let Some(b) = self.repo.fetch(p.args[0]) {
          println!("holy moly!");
          let _ = b.handle_message(&ctx, &msg, &p).await;
        }
      }
    } else {
      println!("bot message - {}", msg.content);
    }
  
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} bro", ready.user.name);
  }
}




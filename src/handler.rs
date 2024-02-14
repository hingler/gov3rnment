use serenity::{all::{Message, Ready}, async_trait, client::{Context, EventHandler}};

use crate::{args::arg_parser::ArgParser, command::{base_command::BaseCommand, demo::pushup::Pushup}};
use crate::command::command_repo::CommandRepo;
use crate::command::demo::ping::Ping;

pub struct Handler {
  repo: CommandRepo
}

impl Handler {
  pub fn new() -> Self {
    let mut repo = CommandRepo::new();
    repo.insert(String::from("ping"), Box::new(Ping));
    repo.insert("pushup".to_owned(), Box::new(Pushup));
    return Handler { repo };
  }
}



#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    // wrap in some sort of similar argument handler
    println!("{}", msg.content);
    
    let token = "g";

    let parser = ArgParser::new(&token, &msg.content);

    // self is considered immutable :(
  
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
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} bro", ready.user.name);
  }
}




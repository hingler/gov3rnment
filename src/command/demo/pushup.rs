use serenity::all::Message;
use serenity::async_trait;
use serenity::client::Context;

use crate::command::base_command::BaseCommand;
use crate::args::arg_parser::ArgParser;

pub struct Pushup;

#[async_trait]
impl BaseCommand for Pushup {
  async fn handle_message(&self, ctx: &Context, msg: &Message, parsed_args: &ArgParser) {
    if parsed_args.args.len() > 1 {
      // turbofish - specify gen
      let num_string = parsed_args.args[1].parse::<i64>();
      let mut resp: String = String::new();
      if let Ok(ns) = num_string {
        resp.push_str("that's cool but i can do ");
  
        resp.push_str((ns + 1).to_string().as_str());
        resp.push_str(" pushups");

      } else {
        println!("bad data received");
        resp.push_str("no shot");
      }


      if let Err(why) = msg.channel_id.say(&ctx.http, resp).await {
        println!("why? {why:?}");
      }
    }
  }
}
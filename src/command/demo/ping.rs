use serenity::all::Message;
use serenity::async_trait;
use serenity::client::Context;

use crate::command::base_command::BaseCommand;
use crate::args::arg_parser::ArgParser;

pub struct Ping;


#[async_trait]
impl BaseCommand for Ping {
  async fn handle_message(&self, ctx: &Context, msg: &Message, parsed_args: &ArgParser) {
    println!("preparing to pong...");
    if parsed_args.args.len() > 0 && parsed_args.args[0] == "ping" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
        println!("Error sending message - {why:?}");
      }
    }
  }
}
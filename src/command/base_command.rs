use serenity::{all::Message, async_trait, client::Context};

use crate::args::arg_parser::ArgParser;

// gonna register manually lole


// just make the claim that we're thread-safe :3

// ah i see - send + sync just enforces that implementers can be auto-send + sync

#[async_trait]
pub trait BaseCommand: Send + Sync {
  async fn handle_message(&self, ctx: &Context, msg: &Message, args: &ArgParser);
}
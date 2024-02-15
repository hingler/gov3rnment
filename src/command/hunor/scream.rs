use serenity::{all::Message, async_trait, client::Context};

use crate::{args::arg_parser::ArgParser, command::base_command::BaseCommand};

use rand::seq::SliceRandom;

const SCREAMS: [&'static str; 3] = [
  "https://tenor.com/view/luigi-toystory-mrpotatohead-creepypasta-gif-14226543197384412466",
  "https://tenor.com/view/maltigi-scary-evil-luigi-creepy-luigi-exe-gif-16926629064327547242",
  "https://tenor.com/view/maltigi-scary-scary-luigi-luigi-luigi-exe-gif-11707553430129539193"
];

pub struct Scream;

#[async_trait]
impl BaseCommand for Scream {
  async fn handle_message(&mut self, ctx: &Context, msg: &Message, _: &ArgParser) {
    // static screams list
    let scream: &'static str = SCREAMS.choose(&mut rand::thread_rng()).unwrap();
    if let Err(why) = msg.channel_id.say(&ctx.http, scream).await {
      println!("scream failed: {why:?}");
    }
  }
}
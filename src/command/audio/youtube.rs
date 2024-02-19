use serenity::{all::{ChannelId, GuildId, Message}, async_trait, client::Context};
use songbird::{input::YoutubeDl, typemap::TypeMapKey};

use reqwest::Client as HttpClient;

use crate::{args::arg_parser::ArgParser, command::base_command::BaseCommand};

use super::audio_wrap::{AudioWrap, SongbirdWrap};

pub struct YTCommands;

pub struct HttpKey;

impl TypeMapKey for HttpKey {
  type Value = HttpClient;
}

// enqueue
// connect
// bounce track req to audiowrap
// also: remove argparser down the line? lol4

// thanks tutoral :3

#[async_trait]
impl BaseCommand for YTCommands {
  async fn handle_message(&self, ctx: &Context, msg: &Message, parsed_args: &ArgParser) {
    // register as yt prob
    // - check parser for a url (args[1])
    // - if valid: play on connect

    
    if parsed_args.args.len() < 2 {
      // do nothing
      if let Err(why) = msg.channel_id.say(&ctx.http, "link not given :/").await {
        println!("error occured in yt command: {why:?}");
        return;
      }
    }

    let id: GuildId;
    let channel_id: ChannelId;
    let url = parsed_args.args[1];
    println!("url: {url:}");

    {
      // cacheref was causing the send issue - scope btwn awaits to avoid
      // store in this closure, copy to sendable vars, then exit
      // (issue is instancingx on one side of an await, then closing on the other)
      let guild = msg.guild(&ctx.cache).unwrap();
      
      channel_id = guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id).expect("user not connected to voice channel");
      id = guild.id;
    }

    let data = ctx.data.read().await;
    let client = data.get::<HttpKey>().cloned().expect("registered");

    
    let dl = YoutubeDl::new(client, String::from(url));
    // issue is sends going out of scope after this await, afaik
    let wrap = SongbirdWrap;
    wrap.connect(ctx, id.clone(), channel_id).await;
    wrap.play(ctx, id, dl.into()).await;

    // tba:
    // - per-guild queueing system (should be easy)
    //   - arc for each player w mutex
    //   - on play: check if resource is in use
  }


}
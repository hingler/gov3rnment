// thinking: context-tied, so init on receive

use serenity::async_trait;
use serenity::prelude::Context;
use serenity::model::id::{ChannelId, GuildId};
use songbird::{input::Input, Event, EventContext, EventHandler, TrackEvent};

pub struct SongbirdWrap;

struct ErrorHandler;

#[async_trait]
pub trait AudioWrap {
  /// Connect to a given guild. (just this for now lole - will handle disconn later :3)
  async fn connect(&self, ctx: &Context, guild: GuildId, channel: ChannelId);

  // play the passed input
  async fn play(&self, ctx: &Context, guild: GuildId, src: Input);
}

#[async_trait]
impl EventHandler for ErrorHandler {
  async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
    match ctx {
      EventContext::Track(track_list) => {
        for (state, handle) in *track_list {
          println!("error encountered in {:?}: {:?}", handle.uuid(), state.playing);
        }
      }

      _ => {
        println!("[unhandled event from songbird :3]");
      }
    }

    return None;
  }
}

#[async_trait]
impl AudioWrap for SongbirdWrap {
  async fn connect(&self, context: &Context, guild: GuildId, channel: ChannelId) {
    let manager = songbird::get(context).await.expect("songbird not available");
    if let Ok(handler_lock) = manager.join(guild, channel).await {
      let mut handler = handler_lock.lock().await;
      handler.add_global_event(TrackEvent::Error.into(), ErrorHandler);
    }
  }

  async fn play(&self, context: &Context, guild: GuildId, src: Input) {
    let manager = songbird::get(context).await.expect("should have been init in main!").clone();
    if let Some(handler_lock) = manager.get(guild) {
      let mut handler = handler_lock.lock().await;
      handler.play_input(src.into());
    }
  }


}
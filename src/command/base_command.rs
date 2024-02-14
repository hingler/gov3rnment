use serenity::{all::Message, async_trait, client::Context};

use crate::args::arg_parser::ArgParser;

use tokio::sync::Mutex;

// gonna register manually lole


// just make the claim that we're thread-safe :3

// ah i see - send + sync just enforces that implementers can be auto-send + sync

#[async_trait]
pub trait BaseCommand: Send + Sync {
  async fn handle_message(&self, ctx: &Context, msg: &Message, args: &ArgParser);
}

pub struct ThreadSafeCommand {
  mutex: Mutex<Box<dyn BaseCommand>>
}

impl ThreadSafeCommand {
  pub fn new(command: Box<dyn BaseCommand>) -> Self {
    let mutex = Mutex::new(command);
    return ThreadSafeCommand { mutex };
  }
}

#[async_trait]
impl BaseCommand for ThreadSafeCommand {
  async fn handle_message(&self, ctx: &Context, msg: &Message, args: &ArgParser) {
    let command_guard = self.mutex.lock().await;
    println!("handling here...");
    command_guard.as_ref().handle_message(ctx, msg, args).await;
  }
}

unsafe impl Send for ThreadSafeCommand {}
unsafe impl Sync for ThreadSafeCommand {}
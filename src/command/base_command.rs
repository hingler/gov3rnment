use serenity::{all::Message, async_trait, client::Context};

use crate::args::arg_parser::ArgParser;

use tokio::sync::Mutex;

// gonna register manually lole


// just make the claim that we're thread-safe :3

// ah i see - send + sync just enforces that implementers can be auto-send + sync

#[async_trait]
pub trait BaseCommand: Send + Sync {
  async fn handle_message(&mut self, ctx: &Context, msg: &Message, args: &ArgParser);
}

pub struct ThreadSafeCommand {
  mutex: Mutex<Box<dyn BaseCommand>>
}

impl ThreadSafeCommand {
  pub fn new(command: Box<dyn BaseCommand>) -> Self {
    let mutex = Mutex::new(command);
    return ThreadSafeCommand { mutex };
  }

  pub async fn handle_message<'a>(&self, ctx: &'a Context, msg: &'a Message, args: &ArgParser<'a>) {
    let mut command_guard = self.mutex.lock().await;
    println!("handling here...");

    // can call in a mutable ctx!!! (lock strips it!)
    command_guard.as_mut().handle_message(ctx, msg, args).await;
  }
}

unsafe impl Send for ThreadSafeCommand {}
unsafe impl Sync for ThreadSafeCommand {}
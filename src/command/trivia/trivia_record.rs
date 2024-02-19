use std::sync::Arc;

use serenity::{all::Message, async_trait, client::Context};

use crate::{args::arg_parser::ArgParser, command::base_command::BaseCommand, db::repo::TriviaRepo};

pub struct TriviaRecord {
  repo: Arc<TriviaRepo>
}

impl TriviaRecord {
  pub fn new(repo: &Arc<TriviaRepo>) -> Self {
    return TriviaRecord { repo: repo.clone() };
  }
}

#[async_trait]
impl BaseCommand for TriviaRecord {
  async fn handle_message(&self, ctx: &Context, msg: &Message, _: &ArgParser) {
    let resp: String;
    if let Some(res) = self.repo.fetch(&msg.author) {
      println!("retrieved user data");
      resp = format!("Correct: {} \nTotal: {} \nAcc: {}", res.correct, res.answers, 100.0 * (res.correct as f64) / (res.answers as f64));
    } else {
      println!("no data on record");
      resp = String::from("No trivia data on record!");
    }
    if let Err(e) = msg.channel_id.say(&ctx.http, resp).await {
      println!("failed to send trivia record - {}", e);
    }
  }
}
use std::{thread::sleep, time::Duration};

use serenity::{all::{Embed, Message}, async_trait, builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage}, client::Context};
use tokio::time::sleep as sleep_async;

use crate::{args::arg_parser::ArgParser, command::{audio::youtube::HttpKey, base_command::BaseCommand}};

use super::trivia_fetcher::TriviaFetcher;

pub struct Trivia;

// fetch a question from the trivia db
// use ctx to send the q
// sleep
// check reactions

#[async_trait]
impl BaseCommand for Trivia {
  async fn handle_message(&mut self, ctx: &Context, msg: &Message, args: &ArgParser) {
    let type_map = ctx.data.read().await;
    let http = type_map.get::<HttpKey>().expect("should be inserted by main func");
    let fetcher = TriviaFetcher::new(http);
    let res = fetcher.fetch_trivia().await;

    if let Err(e) = res {
      println!("couldn't fetch trivia q! {}", e);
      return;
    }

    let question = res.unwrap();

    println!("received q: {} -> {}", question.question, question.correct_answer);

    // build message
    let channel = msg.channel_id;

    let mut embed = CreateEmbed::new();

    embed = embed.author(CreateEmbedAuthor::new("govbot"));
    embed = embed.description(question.question);

    let mut create_message = CreateMessage::new();

    create_message = create_message.add_embed(embed);
    if let Err(e) = channel.send_message(&ctx.http, create_message).await {
      println!("trivia q failed - {}", e);
    }
    // fetch trivia

    sleep_async(Duration::from_millis(2000)).await;

    if let Err(e) = channel.say(&ctx.http, String::from("g")).await {
      println!("eventual response failed - {}", e);
    }

    // right now: we need to sleep, then get reactions on our post, then respond again (does that work??)
  }
}
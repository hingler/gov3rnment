
use std::env;

use command::audio::youtube::HttpKey;
use serenity::{all::GatewayIntents, Client};
use songbird::SerenityInit;

use reqwest::Client as HttpClient;

mod handler;
mod args;
mod command;
mod db;



use handler::Handler;

// async trait: async fn inside traits (trait defn or impl like here)

// throw some stuff on this as we go

// arrangement: ehh idrc
// - event handler hgets events from serenity
// - pass events down to command handlers (like they were doing prior)
// infra
// - prefix
// - command
// - args

// impl
// - write a tiny parser for it
// - prefix, command, args, as well as remaining message content
// treat items in quotation as a single arg (tba i think lole)

// impl2
// - incokming message reaches some central cmpt
// - cmpt parses via argparser opt
// - if return: send to arg0
// - if not: send to general listeners



#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("MISSING TOKEN");
  let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
  let j = Handler::new();

  let mut client = Client::builder(&token, intents)
    .event_handler(j)
    .register_songbird()
    .type_map_insert::<HttpKey>(HttpClient::new())
    .await
    .expect("err!");
  if let Err(why) = client.start().await {
    println!("Client error: {why:?}");
  }
}
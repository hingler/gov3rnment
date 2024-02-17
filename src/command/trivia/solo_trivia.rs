use std::{collections::HashSet, time::Duration};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use serenity::{all::{GuildId, Message, ReactionType, User}, async_trait, builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage}, client::Context};
use tokio::{sync::Mutex, time::sleep as sleep_async};

use crate::{args::arg_parser::ArgParser, command::{audio::youtube::HttpKey, base_command::BaseCommand}};

use super::trivia_fetcher::TriviaFetcher;

pub struct Trivia {
  games: Mutex<HashSet<GuildId>>
}

// fetch a question from the trivia db
// use ctx to send the q
// sleep
// check reactions

const RESPONSE_EMOJIS: [&str; 4] = [
  "🇦",
  "🇧",
  "🇨",
  "🇩"
];

#[async_trait]
impl BaseCommand for Trivia {
  async fn handle_message(&mut self, ctx: &Context, msg: &Message, _: &ArgParser) {
    let gid = if let Some(guild_id) = msg.guild_id {
      guild_id
    } else {
      // test - dm
      println!("guild id could not be inferred - returning");
      return;
    };

    {
      let mut games = self.games.lock().await;
      if games.contains(&gid) {
        println!("game already in progress");
        return;
      }

      games.insert(gid.clone());
    }


    let q_option = self.fetch_trivia(ctx).await;
    let question = if q_option.is_none() {
      println!("q fetch failed - aborting trivia...");
      return;
    } else {
      q_option.unwrap()
    };

    let embed = self.create_embed(&question).await;
    let question_embed = msg.channel_id.send_message(&ctx.http, CreateMessage::new().add_embed(embed)).await;
    let question_message: Message;
    match question_embed {
      Err(e) => {
        println!("trivia q failed - {}", e);
        return;
      }

      Ok(q) => { question_message = q; }
    }

    for idx in 0..question.responses.len() {
      if let Err(e) = question_message.react(&ctx.http, ReactionType::Unicode(String::from(RESPONSE_EMOJIS[idx]))).await {
        println!("reaction add failed! {}", e);
        return;
      }
    }

    // fetch trivia
    sleep_async(Duration::from_millis(5000)).await;

    let time_notif = msg.channel_id.say(&ctx.http, "5 seconds remaining!").await;
    
    sleep_async(Duration::from_millis(5000)).await;

    if !time_notif.is_err() {
      let _ = time_notif.unwrap().delete(&ctx.http).await;
    }
    


    let responses = self.fetch_trivia_reactions(ctx, &question_message).await;
    if let None = responses {
      println!("something happened to question message: aborting trivia");
      return;
    }

    println!("g");

    let results = responses.expect("non-error case");
    let correct_users = self.get_correct_respondants(question.correct_idx, &results).await;

    let mut answer_message = String::new();


    answer_message.push_str("the answer was ");
    answer_message.push_str(&question.responses[question.correct_idx]);
    answer_message.push('\n');
    answer_message.push('\n');

    let mut user_nicks: Vec<String> = Vec::new();
    for user in correct_users {
      let nick = user.nick_in(&ctx.http, gid).await.unwrap_or(user.name);
      user_nicks.push(nick);
    }
    
    let user_list = user_nicks.join(", ");

    if user_list.len() > 0 {
      answer_message.push_str("Congratulations to ");
      answer_message.push_str(user_list.as_str());
      answer_message.push_str(" for answering correctly!");
    } else {
      answer_message.push_str("no one answered correctly :/");
    }

    if let Err(e) = msg.channel_id.say(&ctx.http, answer_message).await {
      println!("failed to send results: {}", e);
    }

    {
      let mut games = self.games.lock().await;
      games.remove(&gid);
    }
    // right now: we need to sleep, then get reactions on our post, then respond again (does that work??)
  }
}

struct DiscordTriviaQuestion {
  question: String,
  responses: Vec<String>,
  correct_idx: usize
}

struct DiscordTriviaResponses {
  responses: [Vec<User>; 4]
}

impl DiscordTriviaResponses {
  fn new() -> Self {
    return DiscordTriviaResponses { responses: [vec![], vec![], vec![], vec![]] };
  }
}

// trivia parts
impl Trivia {
  pub fn new() -> Self {
    return Trivia {
      games: Mutex::new(HashSet::new())
    }
  }
  // - coerce response into form we want (answers merged, with correct idx stored)
  async fn fetch_trivia(&self, ctx: &Context) -> Option<DiscordTriviaQuestion> {
    let type_map = ctx.data.read().await;
    let http = type_map.get::<HttpKey>().expect("should be inserted by main func");
    let fetcher = TriviaFetcher::new(http);
    let res = fetcher.fetch_trivia().await;

    if let Err(e) = res {
      println!("couldn't fetch trivia q! {}", e);
      return None;
    }

    let question = res.unwrap();

    println!("received q: {} -> {}", question.question, question.correct_answer);

    // get answers and correct index
    let mut answers = question.incorrect_answers.clone();
    answers.push(question.correct_answer.clone());
    answers.shuffle::<ThreadRng>(&mut thread_rng());
    let correct_index = answers.iter().position(|s: &String| s.eq(&question.correct_answer)).expect("should already in array");

    return Some(DiscordTriviaQuestion { question: question.question, responses: answers, correct_idx: correct_index });
  }
  // - create embed and send message (return sent message as an option)
  async fn create_embed(&self, question: &DiscordTriviaQuestion) -> CreateEmbed {
    let mut embed_string = String::new();
    embed_string.push_str(&question.question);
    embed_string.push('\n');

    for idx in 0..question.responses.len() {
      let uni_char = RESPONSE_EMOJIS[idx];
      embed_string.push_str(uni_char);
      embed_string.push_str(": ");
      embed_string.push_str(question.responses.get(idx).unwrap());
      embed_string.push('\n');
    }

    let mut embed = CreateEmbed::new();
    embed = embed.author(CreateEmbedAuthor::new("govbot"));
    embed = embed.description(&embed_string);

    return embed;
  }

  // - coerce response reactions into user vec
  async fn fetch_trivia_reactions(&self, ctx: &Context, sent_message: &Message) -> Option<DiscordTriviaResponses> {
    let msg = ctx.http.get_message(sent_message.channel_id, sent_message.id).await;
    let updated_message = if msg.is_err() {
      println!("message re-fetch failed!");
      return None;
    } else {
      msg.unwrap()
    };

    println!("fetched updated message");

    let mut responses = DiscordTriviaResponses::new();

    for idx in 0..RESPONSE_EMOJIS.len() {
      let emoji = RESPONSE_EMOJIS[idx];
      let users = updated_message.reaction_users(&ctx.http, ReactionType::Unicode(String::from(emoji)), None, None).await;
      if let Ok(user_vec) = users {
        println!("{} for {}", user_vec.len(), emoji);
        let output = responses.responses.get_mut(idx).unwrap();
        
        for user in user_vec.iter() {
          if !user.bot {
            output.push(user.clone());
          }
        }
        
      }
    }

    println!("returning responses!");

    return Some(responses);
  }

  async fn get_correct_respondants(&self, idx: usize, responses: &DiscordTriviaResponses) -> Vec<User> {
    // everyone in correct idx gets moved to vec
    // if incorrect user is in correct vec, then remove

    println!("fetching respondants");

    let mut correct_users = responses.responses[idx].clone();
    println!("correct users: {}", correct_users.len());
    for i in 0..RESPONSE_EMOJIS.len() {
      if i == idx { continue };

      println!("handling for i = {}", i);

      let incorrect_vec = &responses.responses[i];
      for user in incorrect_vec {
        // if a correct user is found in the incorrect vec, then remove them
        if let Some(pos) = correct_users.iter().position(|u: &User| u.eq(user)) {
          correct_users.remove(pos);
          println!("removed {}", pos);
        }
      }
    }

    println!("returning correct users...");

    return correct_users;
  }
}
// - filter response reactions to get a list of correct answers
use reqwest::Client;
use serde::Deserialize;
use html_escape::decode_html_entities;



use serde_json;

const TRIVIA_URL: &'static str = "https://opentdb.com/api.php?amount=1";

pub struct TriviaFetcher<'a> {
  client: &'a Client
}


#[derive(Deserialize, Clone)]
pub struct TriviaQuestion {
  pub r#type: String,
  pub difficulty: String,
  pub category: String,
  pub question: String,
  pub correct_answer: String,
  pub incorrect_answers: Vec<String>
}

#[derive(Deserialize)]
pub struct TriviaResponse {
  response_code: i32,
  results: Vec<TriviaQuestion>
}

impl <'a> TriviaFetcher<'a> {
  pub fn new(c: &'a Client) -> Self {
    return TriviaFetcher {
      client: c
    };
  }

  pub async fn fetch_trivia(&self) -> Result<TriviaQuestion, String> {
    let fetch = self.client.get(TRIVIA_URL).send().await;
    if let Err(e) = fetch {
      println!("{}", e);
      return Err(e.to_string());
    }

    let resp = fetch.expect("error case handled").text().await;

    let text;
    match resp {
      Ok(t) => text = t,
      Err(e) => return Err(e.to_string())
    }

    let parse_result = serde_json::from_str::<TriviaResponse>(text.as_str());

    let res: TriviaResponse;
    match parse_result {
      Ok(t) => res = t,
      Err(e) => {
        println!("parsing failed! {}", e);
        return Err(e.to_string());
      }
    }

    if res.response_code != 0 {
      return Err(String::from("response not available due to API error!"));
    }

    let mut q = res.results[0].clone();
    q.r#type = String::from(decode_html_entities(&q.r#type));
    q.question = String::from(decode_html_entities(&q.question));
    q.correct_answer = String::from(decode_html_entities(&q.correct_answer));

    for i in 0..q.incorrect_answers.len() {
      q.incorrect_answers[i] = String::from(decode_html_entities(&q.incorrect_answers[i]));
    }
    
    return Ok(q);
  }
}
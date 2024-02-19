use std::sync::{Arc, Mutex};

use rusqlite::{Connection, Error};
use serenity::all::User;



pub struct TriviaRepo {
  conn: Arc<Mutex<Box<Connection>>>,
  valid: bool
}

#[derive(Debug, Clone)]
pub struct TriviaData {
  pub correct: usize,
  pub answers: usize
}

impl TriviaRepo {
  pub fn new(conn: &Arc<Mutex<Box<Connection>>>) -> Self {

    let valid: bool;
    {
      let conn = conn.lock().expect("panic if poisoned");
      let res = conn.execute("CREATE TABLE IF NOT EXISTS trivia (
        id        INTEGER PRIMARY KEY,
        correct   INTEGER NOT NULL,
        total     INTEGER NOT NULL
      )", ());

      valid = res.is_ok();
    }

    return TriviaRepo {
      conn: conn.clone(),
      valid
    };
  }

  fn check_valid(&self) -> Option<()> {
    return match self.valid {
        true => Some(()),
        false => None,
    }
  }

  pub fn fetch(&self, user: &User) -> Option<TriviaData> {
    self.check_valid()?;
    let id = user.id.get();
    let conn = self.conn.lock().expect("panic if poisoned");
    let mut statement = conn.prepare("SELECT id, correct, total FROM trivia WHERE id = :id;").ok()?;
    
    let results = statement.query_map(&[(":id", &id)], |row| {
      return Ok(TriviaData {
        correct: row.get(1)?,
        answers: row.get(2)?
      });
    }).ok()?;

    for result in results {
      return result.ok();
    }

    return None;
  }

  // there's definitely some way to yupdate this
  // - tag primary key via attrib
  // - tag class via token stream
  // - create: get a table name, map stringified types to SQL types
  // - fetch: receive primary key, select (field names macro'd from struct) where (primary key name) = (placeholder)
  // - 

  pub fn record_response(&self, user: &User, is_correct: bool) -> Result<(), Error> {
    let mut data: TriviaData;
    if let Some(d) = self.fetch(user) {
      data = d;
    } else {
      data = TriviaData { correct: 0, answers: 0 };
    }

    data.correct += if is_correct { 1 } else { 0 };
    data.answers += 1;


    let conn = self.conn.lock().expect("panic if poisoned");
    let q_res = conn.execute("REPLACE INTO trivia (id, correct, total) VALUES (?1, ?2, ?3)", (user.id.get(), data.correct, data.answers))?;

    println!("rows modified by response: {}", q_res);
    return Ok(());
  }

  // not having fun writing this rn
  // gonna do something else instead


}
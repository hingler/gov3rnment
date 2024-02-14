use std::vec::Vec;

pub struct ArgParser<'a> {
  pub message: &'a String,
  pub token: &'a str,
  pub args: Vec<&'a str>

}

impl<'a> ArgParser<'a> {
  pub fn new(token: &'static str, message: &'a String) -> Option<Self> {

    if message.starts_with(token) {
      let msg_slice = message.strip_prefix(token).unwrap().trim();

      let args = msg_slice.split(' ').collect();
      return Some(ArgParser{ message, token, args });
    }

    return None;
  }
}
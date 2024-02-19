
use std::collections::HashMap;
use crate::command::base_command::BaseCommand;
// repo for storing commands

pub struct CommandRepo {
  commands: HashMap<String, Box<dyn BaseCommand>>
}

impl CommandRepo {
  pub fn new() -> Self {
    let commands = HashMap::new();
    return CommandRepo{ commands };
  }

  pub fn insert(&mut self, command: String, handler: Box<dyn BaseCommand>) {
    self.commands.insert(command, handler);
  }

  pub fn fetch(&self, command: &str) -> Option<&Box<dyn BaseCommand>> {
    return self.commands.get(command);
  }


}


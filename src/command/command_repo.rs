
use std::collections::HashMap;
use crate::command::base_command::BaseCommand;
use crate::command::base_command::ThreadSafeCommand;

// repo for storing commands

pub struct CommandRepo {
  commands: HashMap<String, ThreadSafeCommand>
}

impl CommandRepo {
  pub fn new() -> Self {
    let commands = HashMap::new();
    return CommandRepo{ commands };
  }

  pub fn insert(&mut self, command: String, handler: Box<dyn BaseCommand>) {
    self.commands.insert(command, ThreadSafeCommand::new(handler));
  }

  pub fn fetch(&self, command: &str) -> Option<&ThreadSafeCommand> {
    return self.commands.get(command);
  }


}


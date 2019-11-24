use super::{BufferState, Canceled, Stopped, Transformer};
use crate::keyboards::KeyCode;
use std::collections::HashSet;
use BufferState::*;

#[derive(Clone, Debug)]
pub struct DirectTransformer {
  buffer: String,
  buffer_state: BufferState,
}

impl DirectTransformer {
  pub fn new() -> Self {
    DirectTransformer {
      buffer: "".to_string(),
      buffer_state: Continue,
    }
  }
}

impl Transformer for DirectTransformer {
  fn is_stopped(&self) -> bool {
    self.buffer_state == Stop
  }

  fn push_character(&mut self, character: char) -> Box<dyn Transformer> {
    if self.buffer_state == Stop {
      return Box::new(Stopped::new(self.buffer.clone()));
    }

    self.buffer_state = Stop;
    self.buffer.push(character);

    return Box::new(Stopped::new(self.buffer.clone()));
  }
  fn push_key_code(&self, _: HashSet<KeyCode>, key_code: &KeyCode) -> Box<dyn Transformer> {
    match key_code {
      KeyCode::Escape => Box::new(Canceled::new()),
      _ => Box::new(self.clone()),
    }
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

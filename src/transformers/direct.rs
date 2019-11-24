use super::aspect::Stopped;
use super::BufferState::*;
use super::{BufferState, Transformer};

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

  fn cancel(&mut self) -> Box<dyn Transformer> {
    self.buffer_state = Stop;
    self.buffer = "".to_string();

    return Box::new(Stopped::new("".to_string()));
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

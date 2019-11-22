use super::BufferState::*;
use super::{BufferState, Transformer};

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

  fn push(&mut self, character: char) {
    if self.buffer_state == Stop {
      return;
    }

    self.buffer_state = Stop;
    self.buffer.push(character);
  }

  fn cancel(&mut self) -> String {
    self.buffer_state = Stop;

    std::mem::replace(&mut self.buffer, "".to_string())
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

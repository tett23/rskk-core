use super::tables::hiragana_convert;
use super::BufferState::*;
use super::{BufferState, Transformer};

pub struct HiraganaTransformer {
  buffer: String,
  buffer_state: BufferState,
}

impl HiraganaTransformer {
  pub fn new() -> Self {
    HiraganaTransformer {
      buffer: "".to_string(),
      buffer_state: Continue,
    }
  }
}

impl Transformer for HiraganaTransformer {
  fn is_stopped(&self) -> bool {
    self.buffer_state == Stop
  }

  fn push(&mut self, character: char) {
    if self.buffer_state == Stop {
      return;
    }

    if let Some((c, cont)) = hiragana_convert(&self.buffer, character) {
      self.buffer_state = cont;
      std::mem::replace(&mut self.buffer, c);
    } else {
      std::mem::replace(&mut self.buffer, character.to_string());
    }
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

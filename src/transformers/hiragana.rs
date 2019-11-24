use super::aspect::Stopped;
use super::tables::hiragana_convert;
use super::BufferState::*;
use super::{BufferState, Transformer};

#[derive(Clone, Debug)]
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

  pub fn new_from(buffer: String, buffer_state: BufferState) -> Self {
    HiraganaTransformer {
      buffer,
      buffer_state,
    }
  }
}

impl Transformer for HiraganaTransformer {
  fn is_stopped(&self) -> bool {
    self.buffer_state == Stop
  }

  fn push_character(&mut self, character: char) -> Box<dyn Transformer> {
    if self.buffer_state == Stop {
      return Box::new(Self::new_from(
        self.buffer.clone(),
        self.buffer_state.clone(),
      ));
    }

    if let Some((new_buffer, new_buffer_state)) = hiragana_convert(&self.buffer, character) {
      Box::new(Self::new_from(new_buffer, new_buffer_state))
    } else {
      self.cancel()
    }
  }

  fn cancel(&mut self) -> Box<dyn Transformer> {
    self.buffer_state = Stop;
    self.buffer = "".to_string();

    Box::new(Stopped::new("".to_string()))
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

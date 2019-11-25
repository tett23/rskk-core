use super::super::{Transformer, TransformerState, TransformerTypes};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Stopped {
  buffer: String,
}

impl Stopped {
  pub fn new(buffer: String) -> Self {
    Stopped { buffer }
  }
}

impl TransformerState for Stopped {
  fn is_stopped(&self) -> bool {
    true
  }
}

impl Transformer for Stopped {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Stopped
  }

  fn push_character(&self, _: char) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.buffer_content()))
  }

  fn push_key_code(&self, _: &HashSet<KeyCode>, _: &KeyCode) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.buffer_content()))
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

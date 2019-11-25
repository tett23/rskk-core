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

  pub fn empty() -> Self {
    Stopped {
      buffer: "".to_string(),
    }
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

  fn try_change_transformer(&self, _: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    None
  }

  fn push_character(&self, _: char) -> Box<dyn Transformer> {
    Box::new(self.clone())
  }

  fn push_key_code(&self, _: &KeyCode) -> Box<dyn Transformer> {
    Box::new(self.clone())
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

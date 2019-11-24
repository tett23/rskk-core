use super::super::{Transformer, TransformerTypes};
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

impl Transformer for Stopped {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Stopped
  }

  fn is_stopped(&self) -> bool {
    true
  }

  fn push_character(&mut self, _: char) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.buffer_content()))
  }

  fn push_key_code(&self, _: HashSet<KeyCode>, _: &KeyCode) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.buffer_content()))
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

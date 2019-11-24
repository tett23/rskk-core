use super::super::{Transformer, TransformerTypes};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Canceled {}

impl Canceled {
  pub fn new() -> Self {
    Canceled {}
  }
}

impl Transformer for Canceled {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Canceled
  }

  fn is_stopped(&self) -> bool {
    true
  }

  fn push_character(&self, _: char) -> Box<dyn Transformer> {
    Box::new(Canceled::new())
  }

  fn push_key_code(&self, _: HashSet<KeyCode>, key_code: &KeyCode) -> Box<dyn Transformer> {
    Box::new(match key_code {
      _ => Canceled::new(),
    })
  }

  fn buffer_content(&self) -> String {
    "".to_string()
  }

  fn display_string(&self) -> String {
    "".to_string()
  }
}

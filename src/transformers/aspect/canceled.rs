use super::super::{Displayable, KeyImputtable, Transformer, TransformerState, TransformerTypes};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Canceled {}

impl Canceled {
  pub fn new() -> Self {
    Canceled {}
  }
}

impl TransformerState for Canceled {
  fn is_stopped(&self) -> bool {
    true
  }
}

impl Transformer for Canceled {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Canceled
  }
}

impl KeyImputtable for Canceled {
  fn try_change_transformer(&self, _: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    None
  }

  fn push_character(&self, _: char) -> Box<dyn Transformer> {
    Box::new(Canceled::new())
  }

  fn push_key_code(&self, _: &KeyCode) -> Box<dyn Transformer> {
    Box::new(Canceled::new())
  }
}

impl Displayable for Canceled {
  fn buffer_content(&self) -> String {
    "".to_string()
  }

  fn display_string(&self) -> String {
    "".to_string()
  }
}

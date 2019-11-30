use super::super::{
  Config, Displayable, KeyInputtable, Transformer, TransformerState, TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Canceled {
  config: Config,
}

impl Canceled {
  pub fn new(config: Config) -> Self {
    Canceled { config }
  }
}

impl WithConfig for Canceled {
  fn config(&self) -> Config {
    self.config.clone()
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

impl KeyInputtable for Canceled {
  fn try_change_transformer(&self, _: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    None
  }

  fn push_character(&self, _: char) -> Box<dyn Transformer> {
    Box::new(Canceled::new(self.config()))
  }

  fn push_key_code(&self, _: &KeyCode) -> Box<dyn Transformer> {
    Box::new(Canceled::new(self.config()))
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

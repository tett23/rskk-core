use super::super::{
  AsTransformerTrait, Config, Displayable, Transformable, TransformerState, TransformerTypes,
  WithConfig,
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

impl Transformable for Canceled {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Canceled
  }

  fn try_change_transformer(
    &self,
    _: &HashSet<KeyCode>,
    _: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    None
  }

  fn push_character(&self, _: char) -> Box<dyn Transformable> {
    Box::new(self.clone())
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

impl AsTransformerTrait for Canceled {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

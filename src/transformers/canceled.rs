use super::{
  AsTransformerTrait, Config, Displayable, Transformable, TransformerState, TransformerTypes,
  WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct CanceledTransformer {
  config: Config,
}

impl CanceledTransformer {
  pub fn new(config: Config) -> Self {
    CanceledTransformer { config }
  }
}

impl WithConfig for CanceledTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for CanceledTransformer {
  fn is_stopped(&self) -> bool {
    true
  }
}

impl Transformable for CanceledTransformer {
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

impl Displayable for CanceledTransformer {
  fn buffer_content(&self) -> String {
    "".to_string()
  }

  fn display_string(&self) -> String {
    "".to_string()
  }
}

impl AsTransformerTrait for CanceledTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

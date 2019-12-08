use super::{
  AsTransformerTrait, Config, Displayable, KeyCode, Transformable, TransformerTypes, WithConfig,
};

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct StoppedTransformer {
  config: Config,
  buffer: String,
}

impl StoppedTransformer {
  pub fn new(config: Config, buffer: String) -> Self {
    StoppedTransformer { config, buffer }
  }

  pub fn empty(config: Config) -> Self {
    StoppedTransformer {
      config,
      buffer: "".to_string(),
    }
  }
}

impl WithConfig for StoppedTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for StoppedTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Stopped
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

impl Displayable for StoppedTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

impl AsTransformerTrait for StoppedTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

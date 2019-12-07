use super::{
  AsTransformerTrait, Config, Displayable, KeyCode, Transformable, TransformerState,
  TransformerTypes, WithConfig,
};

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Stopped {
  config: Config,
  buffer: String,
}

impl Stopped {
  pub fn new(config: Config, buffer: String) -> Self {
    Stopped { config, buffer }
  }

  pub fn empty(config: Config) -> Self {
    Stopped {
      config,
      buffer: "".to_string(),
    }
  }
}

impl WithConfig for Stopped {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for Stopped {
  fn is_stopped(&self) -> bool {
    true
  }
}

impl Transformable for Stopped {
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

impl Displayable for Stopped {
  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

impl AsTransformerTrait for Stopped {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

use super::{
  AsTransformerTrait, Config, Displayable, KeyCode, Transformable, TransformerTypes, WithConfig,
};

use std::collections::HashSet;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum StoppedReason {
  Compleated,
  Canceled,
}

#[derive(Clone, Debug)]
pub struct StoppedTransformer {
  config: Config,
  reason: StoppedReason,
  buffer: String,
}

impl StoppedTransformer {
  pub fn new<S: Into<String>>(config: Config, reason: StoppedReason, buffer: S) -> Self {
    StoppedTransformer {
      config,
      reason,
      buffer: buffer.into(),
    }
  }

  pub fn completed<S: Into<String>>(config: Config, buffer: S) -> Self {
    Self::new(config, StoppedReason::Compleated, buffer)
  }

  pub fn empty(config: Config) -> Self {
    Self::new(config, StoppedReason::Compleated, "")
  }

  pub fn canceled(config: Config) -> Self {
    Self::new(config, StoppedReason::Canceled, "")
  }
}

impl WithConfig for StoppedTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for StoppedTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Stopped(self.reason)
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

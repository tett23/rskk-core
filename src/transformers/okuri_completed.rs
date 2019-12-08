use super::{
  AsTransformerTrait, Config, Displayable, KeyCode, Transformable, TransformerTypes, WithConfig,
};

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct OkuriCompletedTransformer {
  config: Config,
  yomi: String,
  okuri: String,
}

impl OkuriCompletedTransformer {
  pub fn new<S: Into<String>>(config: Config, yomi: S, okuri: S) -> Self {
    OkuriCompletedTransformer {
      config,
      yomi: yomi.into(),
      okuri: okuri.into(),
    }
  }
}

impl WithConfig for OkuriCompletedTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for OkuriCompletedTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::OkuriCompleted
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

impl Displayable for OkuriCompletedTransformer {
  fn buffer_content(&self) -> String {
    self.yomi.clone() + &self.okuri
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }
}

impl AsTransformerTrait for OkuriCompletedTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

use super::super::{
  AsTransformerTrait, Config, Displayable, KeyCode, Transformable, TransformerState,
  TransformerTypes, WithConfig,
};

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct OkuriCompleted {
  config: Config,
  yomi: String,
  okuri: String,
}

impl OkuriCompleted {
  pub fn new<S: Into<String>>(config: Config, yomi: S, okuri: S) -> Self {
    OkuriCompleted {
      config,
      yomi: yomi.into(),
      okuri: okuri.into(),
    }
  }
}

impl WithConfig for OkuriCompleted {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for OkuriCompleted {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformable for OkuriCompleted {
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

impl Displayable for OkuriCompleted {
  fn buffer_content(&self) -> String {
    self.yomi.clone() + &self.okuri
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }
}

impl AsTransformerTrait for OkuriCompleted {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

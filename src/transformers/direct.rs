use super::{
  AsTransformerTrait, Canceled, Config, Displayable, Stopped, Transformer, TransformerState,
  TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use crate::set;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct DirectTransformer {
  config: Config,
  buffer: String,
}

impl DirectTransformer {
  pub fn new(config: Config) -> Self {
    DirectTransformer {
      config,
      buffer: "".to_string(),
    }
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![TransformerTypes::Hiragana]
  }
}

impl WithConfig for DirectTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for DirectTransformer {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformer for DirectTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Direct
  }

  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self
      .config
      .key_config()
      .try_change_transformer(&Self::allow_transformers(), pressing_keys)
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    return Box::new(Stopped::new(self.config(), character.to_string()));
  }

  fn push_escape(&self) -> Box<dyn Transformer> {
    Box::new(Canceled::new(self.config()))
  }
}

impl Displayable for DirectTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

impl AsTransformerTrait for DirectTransformer {
  fn as_trait(&self) -> Box<dyn Transformer> {
    Box::new(self.clone())
  }
}

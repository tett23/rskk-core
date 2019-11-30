use super::{
  Canceled, Config, Displayable, KeyInputtable, Stopped, Transformer, TransformerState,
  TransformerTypes, WithConfig,
};
use crate::keyboards::{KeyCode, MetaKey};
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
}

impl KeyInputtable for DirectTransformer {
  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self
      .config
      .key_config()
      .try_change_transformer(&Self::allow_transformers(), pressing_keys)
  }

  fn push_key_code(&self, key_code: &KeyCode) -> Box<dyn Transformer> {
    match key_code {
      KeyCode::Meta(MetaKey::Escape) => Box::new(Canceled::new(self.config())),
      _ => Box::new(self.clone()),
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    return Box::new(Stopped::new(self.config(), character.to_string()));
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

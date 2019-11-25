use super::{Canceled, Stopped, Transformer, TransformerState, TransformerTypes};
use crate::keyboards::{KeyCode, MetaKey};
use crate::{set, Config, Dictionary};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct DirectTransformer {
  config: Rc<Config>,
  dictionary: Rc<Dictionary>,
  buffer: String,
}

impl DirectTransformer {
  pub fn new(config: Rc<Config>, dictionary: Rc<Dictionary>) -> Self {
    DirectTransformer {
      config,
      dictionary,
      buffer: "".to_string(),
    }
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![TransformerTypes::Hiragana]
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
      .key_config
      .try_change_transformer(&Self::allow_transformers(), pressing_keys)
  }

  fn push_key_code(&self, key_code: &KeyCode) -> Box<dyn Transformer> {
    match key_code {
      KeyCode::Meta(MetaKey::Escape) => Box::new(Canceled::new()),
      _ => Box::new(self.clone()),
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    return Box::new(Stopped::new(character.to_string()));
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

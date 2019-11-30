use super::tables::hiragana_convert;
use super::{
  AsTransformerTrait, BufferState, Canceled, Config, Displayable, Stopped, Transformer,
  TransformerState, TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use crate::set;
use std::collections::HashSet;
use BufferState::*;

#[derive(Clone, Debug)]
pub struct HiraganaTransformer {
  config: Config,
  buffer: String,
}

impl HiraganaTransformer {
  pub fn new(config: Config) -> Self {
    HiraganaTransformer {
      config,
      buffer: "".to_string(),
    }
  }

  pub fn new_from(&self, buffer: String) -> Self {
    let mut new_state = self.clone();
    new_state.buffer = buffer;

    new_state
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![
      TransformerTypes::Direct,
      TransformerTypes::Henkan,
      TransformerTypes::Abbr,
      TransformerTypes::Katakana,
      TransformerTypes::EnKatakana,
      TransformerTypes::EmEisu
    ]
  }
}

impl WithConfig for HiraganaTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for HiraganaTransformer {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformer for HiraganaTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Hiragana
  }

  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self
      .config
      .key_config()
      .try_change_transformer(&Self::allow_transformers(), pressing_keys)
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    match hiragana_convert(&self.buffer, character) {
      Some((new_buffer, Continue)) => Box::new(self.new_from(new_buffer)),
      Some((new_buffer, Stop)) => Box::new(Stopped::new(self.config(), new_buffer)),
      _ => Box::new(Stopped::empty(self.config())),
    }
  }

  fn push_escape(&self) -> Box<dyn Transformer> {
    Box::new(Canceled::new(self.config()))
  }
}

impl Displayable for HiraganaTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

impl AsTransformerTrait for HiraganaTransformer {
  fn as_trait(&self) -> Box<dyn Transformer> {
    Box::new(self.clone())
  }
}

use super::{
  AsTransformerTrait, AspectTransformer, Config, Displayable, Transformer, TransformerState,
  TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct HenkanTransformer {
  config: Config,
  transformer: Box<dyn Transformer>,
}

impl HenkanTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    HenkanTransformer {
      transformer: Box::new(AspectTransformer::new(config.clone(), transformer_type)),
      config,
    }
  }

  fn new_from_transformer(&self, transformer: Box<dyn Transformer>) -> Self {
    let mut ret = self.clone();
    ret.transformer = transformer;

    ret
  }
}

impl WithConfig for HenkanTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for HenkanTransformer {
  fn is_stopped(&self) -> bool {
    self.transformer.is_stopped()
  }
}

impl Transformer for HenkanTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Henkan
  }

  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self.transformer.try_change_transformer(pressing_keys)
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    let new_transformer = self.transformer.push_character(character);
    if new_transformer.is_stopped() {
      return new_transformer;
    }

    Box::new(self.new_from_transformer(new_transformer))
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformer> {
    let new_transformer = self.transformer.push_meta_key(key_code);
    if new_transformer.is_stopped() {
      return new_transformer;
    }

    Box::new(self.new_from_transformer(new_transformer))
  }
}

impl Displayable for HenkanTransformer {
  fn buffer_content(&self) -> String {
    self.transformer.buffer_content()
  }

  fn display_string(&self) -> String {
    self.transformer.display_string()
  }
}

impl AsTransformerTrait for HenkanTransformer {
  fn as_trait(&self) -> Box<dyn Transformer> {
    Box::new(self.clone())
  }
}

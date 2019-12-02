use super::{
  AsTransformerTrait, AspectTransformer, Config, Displayable, Transformable, TransformerState,
  TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct HenkanTransformer {
  config: Config,
  transformer: Box<dyn Transformable>,
}

impl HenkanTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    HenkanTransformer {
      transformer: Box::new(AspectTransformer::new(config.clone(), transformer_type)),
      config,
    }
  }

  fn new_from_transformer(&self, transformer: Box<dyn Transformable>) -> Self {
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

impl Transformable for HenkanTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Henkan
  }

  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self.transformer.try_change_transformer(pressing_keys)
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let new_transformer = self.transformer.push_character(character);
    if new_transformer.is_stopped() {
      return new_transformer;
    }

    Box::new(self.new_from_transformer(new_transformer))
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformable> {
    let new_transformer = self.transformer.push_meta_key(key_code);

    self.transformer_updated(new_transformer)
  }

  fn transformer_updated(&self, new_transformer: Box<dyn Transformable>) -> Box<dyn Transformable> {
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
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

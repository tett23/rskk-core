use super::{AspectTransformer, Transformer, TransformerTypes};
use crate::keyboards::KeyCode;
use crate::{Config, Dictionary};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct HenkanTransformer {
  config: Rc<Config>,
  dictionary: Rc<Dictionary>,
  transformer: Box<dyn Transformer>,
}

impl HenkanTransformer {
  pub fn new(
    config: Rc<Config>,
    dictionary: Rc<Dictionary>,
    transformer_type: TransformerTypes,
  ) -> Self {
    HenkanTransformer {
      transformer: Box::new(AspectTransformer::new(
        config.clone(),
        dictionary.clone(),
        transformer_type,
      )),
      config,
      dictionary,
    }
  }
}

impl Transformer for HenkanTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Henkan
  }

  fn is_stopped(&self) -> bool {
    self.transformer.is_stopped()
  }

  fn push_character(&mut self, character: char) -> Box<dyn Transformer> {
    self.transformer.push_character(character)
  }

  fn push_key_code(
    &self,
    pressing_keys: HashSet<KeyCode>,
    key_code: &KeyCode,
  ) -> Box<dyn Transformer> {
    self.transformer.push_key_code(pressing_keys, key_code)
  }

  fn buffer_content(&self) -> String {
    self.transformer.buffer_content()
  }

  fn display_string(&self) -> String {
    self.transformer.display_string()
  }
}

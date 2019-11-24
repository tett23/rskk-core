use super::{AspectTransformer, Transformer, TransformerTypes};
use crate::{Config, Dictionary};
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

  fn push(&mut self, character: char) -> Box<dyn Transformer> {
    self.transformer.push(character)
  }

  fn cancel(&mut self) -> Box<dyn Transformer> {
    self.transformer.cancel()
  }

  fn enter(&mut self) -> Box<dyn Transformer> {
    self.transformer.enter()
  }

  fn space(&mut self) -> Box<dyn Transformer> {
    self.transformer.space()
  }

  fn tab(&mut self) -> Box<dyn Transformer> {
    self.transformer.tab()
  }

  fn delete(&mut self) -> Box<dyn Transformer> {
    self.transformer.delete()
  }

  fn buffer_content(&self) -> String {
    self.transformer.buffer_content()
  }

  fn display_string(&self) -> String {
    self.transformer.display_string()
  }
}

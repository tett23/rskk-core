use crate::{CompositionResult, Dictionary, RSKKConfig};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Context {
  config: Rc<RSKKConfig>,
  dictionary: Rc<Dictionary>,
  result: CompositionResult,
}

impl Context {
  pub fn new(config: Rc<RSKKConfig>, dictionary: Rc<Dictionary>) -> Self {
    Self {
      config,
      dictionary,
      result: CompositionResult::new(),
    }
  }

  pub fn new_empty(&self) -> Rc<Self> {
    Rc::new(Self::new(self.config.clone(), self.dictionary.clone()))
  }

  pub fn config(&self) -> &RSKKConfig {
    &self.config
  }

  pub fn dictionary(&self) -> &Dictionary {
    &self.dictionary
  }
}

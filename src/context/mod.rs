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
    Context {
      config,
      dictionary,
      result: CompositionResult::new(),
    }
  }

  pub fn config(&self) -> &RSKKConfig {
    &self.config
  }

  pub fn dictionary(&self) -> &Dictionary {
    &self.dictionary
  }
}

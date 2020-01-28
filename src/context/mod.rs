use std::cell::RefCell;
use std::rc::Rc;

use crate::{CompositionResult, Dictionary, RSKKConfig};

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

  pub fn new_empty(&self) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self::new(
      self.config.clone(),
      self.dictionary.clone(),
    )))
  }

  pub fn config(&self) -> &RSKKConfig {
    &self.config
  }

  pub fn dictionary(&self) -> &Dictionary {
    &self.dictionary
  }

  pub fn result(&self) -> &CompositionResult {
    &self.result
  }

  pub fn push_result_string<S: Into<String>>(&mut self, buffer: S) {
    self.result.push_buffer(buffer)
  }
}

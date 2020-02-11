use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::transformers::Transformable;
use crate::{CompositionResult, Dictionary, DictionaryEntry, RSKKConfig};

#[derive(Clone)]
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

  pub fn copy(&self) -> Rc<RefCell<Self>> {
    let mut ret = Self::new(self.config.clone(), self.dictionary.clone());
    ret.result = self.result.clone();

    Rc::new(RefCell::new(ret))
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

  pub fn push_dictionary_updates(&mut self, updates: &Vec<DictionaryEntry>) {
    self.result.push_dictionary_updates(updates)
  }

  pub fn pop_stopped_buffer(&mut self) {
    self.result.pop_stopped_buffer()
  }

  pub fn merge_result(&mut self, result: &CompositionResult) {
    self.result.merge_result(result)
  }

  pub fn clear_stopped_buffer(&mut self) {
    self.result.clear_stopped_buffer()
  }
}

impl fmt::Debug for Context {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", &self.result)
  }
}

#[derive(Clone, Debug)]
pub struct Contexts(Vec<Rc<RefCell<Context>>>);

impl Contexts {
  pub fn stopped_buffer(&self) -> Option<String> {
    Some(
      self
        .0
        .iter()
        .map(|item| item.borrow().result().stopped_buffer())
        .filter(|item| item.is_some())
        .collect::<Vec<_>>(),
    )
    .and_then(|vec| match vec.is_empty() {
      true => None,
      false => Some(vec),
    })
    .map(|vec| {
      vec.iter().fold(String::new(), |acc, item| {
        acc + (item.as_ref().unwrap_or(&String::new()))
      })
    })
  }

  pub fn dictionary_updates(&self) -> Vec<DictionaryEntry> {
    self
      .0
      .iter()
      .map(|item| {
        item
          .borrow()
          .result()
          .dictionary_updates()
          .iter()
          .map(|item| item.clone())
          .collect::<Vec<_>>()
      })
      .fold(vec![], |mut acc, item| {
        item.iter().for_each(|item| acc.push(item.clone()));
        acc
      })
  }
}

impl From<&Vec<Rc<RefCell<Context>>>> for Contexts {
  fn from(items: &Vec<Rc<RefCell<Context>>>) -> Contexts {
    let mut ret = Contexts(vec![]);
    items.into_iter().for_each(|item| ret.0.push(item.clone()));

    ret
  }
}

impl From<&Vec<Box<dyn Transformable>>> for Contexts {
  fn from(items: &Vec<Box<dyn Transformable>>) -> Contexts {
    Contexts::from(
      &items
        .iter()
        .map(|item| item.clone_context())
        .collect::<Vec<_>>(),
    )
  }
}

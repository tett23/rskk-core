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

  pub fn new_empty(&self) -> Self {
    Self::new(self.config.clone(), self.dictionary.clone())
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

  pub fn push_result_string<S: Into<String>>(&self, buffer: S) -> Self {
    Self {
      result: self.result.push_buffer(buffer),
      ..self.clone()
    }
  }

  pub fn push_dictionary_updates(&self, updates: &Vec<DictionaryEntry>) -> Self {
    Self {
      result: self.result.push_dictionary_updates(updates),
      ..self.clone()
    }
  }

  pub fn pop_stopped_buffer(&self) -> Self {
    Self {
      result: self.result.pop_stopped_buffer(),
      ..self.clone()
    }
  }

  pub fn merge_result(&self, result: &CompositionResult) -> Self {
    Self {
      result: self.result.merge_result(result),
      ..self.clone()
    }
  }

  pub fn clear_stopped_buffer(&self) -> Self {
    Self {
      result: self.result.clear_stopped_buffer(),
      ..self.clone()
    }
  }
}

impl fmt::Debug for Context {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", &self.result)
  }
}

#[derive(Clone, Debug)]
pub struct Contexts(Vec<Context>);

impl Contexts {
  pub fn stopped_buffer(&self) -> Option<String> {
    Some(
      self
        .0
        .iter()
        .map(|item| item.result().stopped_buffer())
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

impl From<&Vec<Context>> for Contexts {
  fn from(items: &Vec<Context>) -> Contexts {
    let mut ret = Contexts(vec![]);
    items.iter().for_each(|item| ret.0.push(item.clone()));

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

#![cfg(test)]

use super::str_to_key_code_vector;
use crate::composition::Composition;
use crate::transformers::{Transformable, TransformerTypes};

#[derive(Debug)]
pub struct Example {
  pub display: Option<String>,
  pub stopped_buffer: Option<String>,
  pub transformer_type: Option<TransformerTypes>,
}

impl Example {
  pub fn new() -> Self {
    Example {
      display: None,
      stopped_buffer: None,
      transformer_type: None,
    }
  }

  pub fn display<S: Into<String>>(&mut self, value: S) {
    self.display = Some(value.into())
  }

  pub fn stopped_buffer<S: Into<String>>(&mut self, value: S) {
    self.stopped_buffer = Some(value.into())
  }

  pub fn transformer_type(&mut self, value: TransformerTypes) {
    self.transformer_type = Some(value)
  }

  pub fn test(&self, tf: &Box<dyn Transformable>) -> Result<(), String> {
    Some(
      vec![
        self.test_display(tf.display_string()),
        self.test_stopped_buffer(tf.clone_context().borrow().result().stopped_buffer()),
        self.test_transformer_type(tf.transformer_type()),
      ]
      .iter()
      .filter(|item| item.is_err())
      .collect::<Vec<_>>(),
    )
    .map(|vec| match vec.is_empty() {
      true => None,
      false => Some(vec),
    })
    .flatten()
    .map(|vec| {
      vec.iter().fold(String::new(), |acc, item| {
        acc + &item.as_ref().err().unwrap()
      })
    })
    .map(|messages| Err(messages))
    .unwrap_or(Ok(()))
  }

  fn test_display(&self, actual: String) -> Result<(), String> {
    self
      .display
      .as_ref()
      .map(|expected| match expected == &actual {
        true => Ok(()),
        false => Err(format!("display: {:?} == {:?}; ", expected, actual)),
      })
      .unwrap_or(Ok(()))
  }

  fn test_stopped_buffer(&self, actual: Option<String>) -> Result<(), String> {
    self
      .stopped_buffer
      .as_ref()
      .map(|expected| match Some(expected) == actual.as_ref() {
        true => Ok(()),
        false => Err(format!(
          "stopped_buffer: {:?} == {:?}; ",
          Some(expected),
          actual
        )),
      })
      .unwrap_or(Ok(()))
  }

  fn test_transformer_type(&self, actual: TransformerTypes) -> Result<(), String> {
    self
      .transformer_type
      .as_ref()
      .map(|expected| match expected == &actual {
        true => Ok(()),
        false => Err(format!(
          "transformer_type: {:?} == {:?}; ",
          expected, actual
        )),
      })
      .unwrap_or(Ok(()))
  }
}

pub struct TestData {
  input: String,
  transformer: Box<dyn Transformable>,
  example: Example,
}

impl TestData {
  pub fn new<S: Into<String>>(
    input: S,
    transformer: Box<dyn Transformable>,
    example: Example,
  ) -> Self {
    TestData {
      input: input.into(),
      transformer,
      example,
    }
  }

  pub fn test(&self) -> Result<(), String> {
    let mut composition = Composition::new_from_transformer(
      self.transformer.clone_context().borrow().new_empty(),
      self.transformer.clone(),
    );
    composition.push_key_events(&str_to_key_code_vector(&self.input));

    self.example.test(&composition.transformer())
  }

  pub fn batch(items: Vec<TestData>) {
    Some(
      items
        .into_iter()
        .enumerate()
        .map(|(i, item)| (i, item.input.clone(), item.test()))
        .filter(|(_, _, item)| item.is_err())
        .collect::<Vec<_>>(),
    )
    .and_then(|list| match list.is_empty() {
      true => None,
      false => Some(list),
    })
    .map(|vec| {
      vec.iter().fold(String::new(), |acc, (i, input, item)| {
        acc
          + &format!(
            "{}: input: {:?}; {}\n",
            i + 1,
            input,
            item.as_ref().err().unwrap()
          )
      })
    })
    .map(|s| panic!(s));
  }
}

#![cfg(test)]

use std::fmt;

use super::str_to_key_code_vector;
use crate::composition::Composition;
use crate::transformers::{Transformable, TransformerTypes};

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

  pub fn test(&self, tf: &Box<dyn Transformable>) -> bool {
    if let Some(t) = &self.transformer_type {
      if t != &tf.transformer_type() {
        return false;
      }
    }

    if let Some(buf) = &self.stopped_buffer {
      if Some(buf)
        != tf
          .clone_context()
          .borrow()
          .result()
          .stopped_buffer()
          .as_ref()
      {
        return false;
      }
    }

    if let Some(display_string) = &self.display {
      if display_string != &tf.display_string() {
        return false;
      }
    }

    true
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

  pub fn test(&self) -> bool {
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
        .filter(|(_, item)| item.test())
        .collect::<Vec<(usize, TestData)>>(),
    )
    .and_then(|list| if list.is_empty() { Some(list) } else { None })
    .map(|vec| {
      vec.iter().fold(String::new(), |acc, (i, item)| {
        acc + &format!("{}: {}\n", i, item)
      })
    })
    .map(|s| panic!(s));
  }
}

impl fmt::Display for TestData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}; ", &self.input)?;

    self.example.transformer_type.as_ref().map(|display| {
      write!(
        f,
        "{:?} = {:?}, ",
        display,
        &self.transformer.transformer_type()
      )
    });

    self.example.stopped_buffer.as_ref().map(|buf| {
      write!(
        f,
        "{:?} = {:?}, ",
        Some(buf),
        self
          .transformer
          .clone_context()
          .borrow()
          .result()
          .stopped_buffer()
          .as_ref()
      )
    });

    self.example.display.as_ref().map(|display| {
      write!(
        f,
        "{:?} = {:?}, ",
        display,
        self.transformer.display_string()
      )
    });

    fmt::Result::Ok(())
  }
}

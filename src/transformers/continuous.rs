use super::{
  AsTransformerTrait, Config, Displayable, Stackable, StoppedTransformer, Transformable,
  TransformerTypes, WithConfig,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::tf;

#[derive(Clone)]
pub struct ContinuousTransformer {
  config: Config,
  current_transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
}

impl ContinuousTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    ContinuousTransformer {
      config: config.clone(),
      current_transformer_type: transformer_type,
      stack: vec![tf!(config, transformer_type)],
    }
  }

  fn stopped_buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .filter(|tf| tf.is_stopped())
      .fold("".to_string(), |acc, tf| acc + &tf.buffer_content())
  }

  fn base_transformer(&self) -> Box<dyn Transformable> {
    tf!(self.config(), self.current_transformer_type)
  }
}

impl WithConfig for ContinuousTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for ContinuousTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Continuous
  }

  fn try_change_transformer(
    &self,
    keyboard: &Box<dyn Keyboard>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    self
      .stack
      .last()?
      .try_change_transformer(keyboard, last_key_code)
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    let mut vec = self.stack.last()?.push_character(character)?;
    if vec.last().map(|last| last.is_stopped()).unwrap_or(false) {
      vec.push(self.base_transformer());
    }

    Some(self.replace_last_element(vec))
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    if self.stack.is_empty() {
      return Some(vec![]);
    }

    Some(self.replace_last_element(self.stack.last()?.push_escape()?))
  }

  fn push_enter(&self) -> Option<Vec<Box<dyn Transformable>>> {
    if self.stack.last()?.is_empty() {
      return Some(vec![box StoppedTransformer::completed(
        self.config(),
        self.stopped_buffer_content(),
      )]);
    }

    let mut tfs = self.stack.last()?.push_enter()?;
    if tfs.last().map(|last| last.is_stopped()).unwrap_or(false) {
      tfs.push(self.base_transformer())
    }

    Some(self.replace_last_element(tfs))
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    if self.is_empty() {
      return Some(vec![]);
    }

    self
      .stack
      .last()?
      .push_backspace()
      .and_then(|tfs| Some(self.replace_last_element(tfs)))
      .or_else(|| {
        let mut tf = self.clone();
        tf.stack.pop();
        let stopped_tf = tf.stack.pop();
        let mut stopped_tf = stopped_tf.unwrap().push_backspace()?;
        tf.stack.append(&mut stopped_tf);
        tf.stack.push(self.base_transformer());

        Some(vec![box tf])
      })
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.push_backspace()
  }

  fn push_space(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.stack.last()?.push_space()?))
  }
}

impl Displayable for ContinuousTransformer {
  fn buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .fold("".to_string(), |acc, tf| acc + &tf.buffer_content())
  }

  fn display_string(&self) -> String {
    self
      .stack
      .iter()
      .fold("".to_string(), |acc, tf| acc + &tf.display_string())
  }
}

impl AsTransformerTrait for ContinuousTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.clone(),
      None => self.to_canceled(),
    }
  }
}

impl Stackable for ContinuousTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.stack.push(item);

    box ret
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();
    let item = ret.stack.pop();

    (box ret, item)
  }

  fn replace_last_element(
    &self,
    items: Vec<Box<dyn Transformable>>,
  ) -> Vec<Box<dyn Transformable>> {
    let mut ret = self.clone();

    ret.stack.pop();
    items.iter().for_each(|item| ret.stack.push(item.clone()));
    if ret.stack.len() == 0 {
      return vec![];
    }

    vec![box ret]
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    self.stack.clone()
  }

  fn child_transformer_type(&self) -> TransformerTypes {
    self.stack.last().unwrap().child_transformer_type()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tds;
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::StoppedReason;
  use StoppedReason::*;
  use TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, ContinuousTransformer, Hiragana;
      ["[escape]", "", Stopped(Canceled)],
      ["[backspace]", "", Stopped(Canceled)],
      ["aa[backspace]", "あ", Continuous],
      ["ak[backspace]", "あ", Continuous],
      ["aa[backspace]i", "あい", Continuous],
      ["ak\n", "あ", Continuous],
      ["hiragana", "ひらがな", Continuous],
      ["hiragana\n", "ひらがな", Stopped(Compleated)],
      ["Kannji", "▽かんじ", Continuous],
      ["Kannji \n", "漢字", Continuous],
      ["Kannji \n\n", "漢字", Stopped(Compleated)],
      ["Kannji \n[backspace]", "漢", Continuous],
      ["Kannji \n[backspace]a", "漢あ", Continuous],
    ];
    test_transformer(items);
  }
}

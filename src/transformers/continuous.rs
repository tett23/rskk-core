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

  pub fn from_buffer<S: Into<String>>(
    config: Config,
    transformer_type: TransformerTypes,
    buffer: S,
  ) -> Self {
    ContinuousTransformer {
      config: config.clone(),
      current_transformer_type: transformer_type,
      stack: vec![box StoppedTransformer::completed(config, buffer)],
    }
  }

  fn stopped_buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .filter(|tf| tf.is_stopped())
      .fold("".to_string(), |acc, tf| acc + &tf.buffer_content())
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
    let tf = self
      .send_target()
      .try_change_transformer(keyboard, last_key_code);

    match tf.clone()?.transformer_type() {
      TransformerTypes::Henkan => tf,
      _ => Some(self.replace_last_element(tf?)),
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    self.replace_last_element(self.send_target().push_character(character))
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    match self.send_target().is_stopped() {
      true => box StoppedTransformer::canceled(self.config()),
      false => self.pop().0,
    }
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    box StoppedTransformer::completed(self.config(), self.stopped_buffer_content())
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    self.pop().0
  }

  fn push_delete(&self) -> Box<dyn Transformable> {
    self.push_backspace()
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
    self.buffer_content()
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

    Box::new(ret)
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();
    if ret.send_target().is_empty() {
      ret.stack.pop();
    }

    match ret.stack.last() {
      Some(tf) => {
        let (tf, pop) = tf.pop();

        (ret.replace_last_element(tf), pop)
      }
      None => (self.to_canceled(), None),
    }
  }

  fn replace_last_element(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();
    match item.is_canceled() {
      true => {
        ret.stack.pop();

        match ret.is_empty() {
          true => ret.to_canceled(),
          false => match ret.is_all_stopped() {
            true => ret.push(tf!(ret.config(), ret.current_transformer_type)),
            false => box ret,
          },
        }
      }
      false => {
        ret.stack.pop();
        ret.stack.push(item);

        match ret.is_all_stopped() {
          true => ret.push(tf!(ret.config(), ret.current_transformer_type)),
          false => box ret,
        }
      }
    }
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    self.stack.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::StoppedReason;
  use crate::{tds, tfe};
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
      ["ak\n", "あ", Stopped(Compleated)],
      ["hiragana", "ひらがな", Continuous],
      ["hiragana\n", "ひらがな", Stopped(Compleated)],
      ["Kannji", "▽かんじ", Henkan],
      ["Kannji \n", "漢字", Stopped(Compleated)],
    ];
    test_transformer(items);
  }

  #[test]
  fn stack() {
    let conf = dummy_conf();

    let tf = tfe!(conf, Continuous; "").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = tfe!(conf, Continuous; "a").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = tfe!(conf, Continuous; "aa").pop().0;
    assert_eq!(tf.transformer_type(), Continuous);
    assert_eq!(tf.buffer_content(), "あ");

    let tf = tfe!(conf, Continuous; "aaa").pop().0;
    assert_eq!(tf.transformer_type(), Continuous);
    assert_eq!(tf.buffer_content(), "ああ");
  }
}

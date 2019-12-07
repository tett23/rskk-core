use super::super::{
  AsTransformerTrait, Canceled, Config, ContinuousTransformer, Displayable, Stackable, Stopped,
  Transformable, TransformerState, TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Word(String, Option<String>);

impl Word {
  pub fn new<S: Into<String>>(yomi: S, okuri: Option<S>) -> Self {
    Word(
      yomi.into(),
      match okuri {
        Some(s) => Some(s.into()),
        None => None,
      },
    )
  }

  pub fn display_string(&self) -> String {
    match &self.1 {
      Some(okuri) => self.0.clone() + "*" + &okuri,
      None => self.0.clone(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct UnknownWord {
  config: Config,
  word: Word,
  stack: Vec<Box<dyn Transformable>>,
}

impl UnknownWord {
  pub fn new(config: Config, word: Word) -> Self {
    UnknownWord {
      config: config.clone(),
      word,
      stack: vec![Box::new(ContinuousTransformer::new(
        config,
        TransformerTypes::Hiragana,
      ))],
    }
  }

  fn new_from_stack(&self, stack: Vec<Box<dyn Transformable>>) -> Self {
    let mut ret = self.clone();
    ret.stack = stack;

    ret
  }

  fn stopped_buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .filter(|tf| tf.is_stopped())
      .fold("".to_string(), |acc, tf| acc + &tf.buffer_content())
  }
}

impl Stackable for UnknownWord {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.new_from_stack(self.stack.clone());

    ret.stack.push(item);

    Box::new(ret)
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.new_from_stack(self.stack.clone());

    let item = ret.stack.pop();

    (Box::new(ret), item)
  }

  fn replace_last_element(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.stack.pop();
    ret.stack.push(item);

    Box::new(ret)
  }
}

impl WithConfig for UnknownWord {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for UnknownWord {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformable for UnknownWord {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::UnknownWord
  }

  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self.stack.last()?.try_change_transformer(pressing_keys)
  }

  fn transformer_changed(
    &self,
    new_transformer: Box<dyn Transformable>,
    key: Option<char>,
  ) -> Box<dyn Transformable> {
    let new_transformer = match new_transformer.transformer_type() {
      TransformerTypes::Henkan => match key {
        Some(character) => new_transformer.push_character(character),
        None => new_transformer,
      },
      _ => new_transformer,
    };

    self.replace_last_element(new_transformer)
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let last_tf = self.stack.last();
    if last_tf.is_none() {
      return Box::new(Stopped::empty(self.config.clone()));
    }
    let last_tf = last_tf.unwrap();

    let new_transformer = last_tf.push_character(character);
    match new_transformer.is_stopped() {
      true => {
        let mut ret = self.clone();
        ret.stack.pop();
        ret.stack.push(new_transformer);
        ret.stack.push(Box::new(ContinuousTransformer::new(
          self.config(),
          TransformerTypes::Hiragana,
        )));

        Box::new(ret)
      }
      false => self.replace_last_element(new_transformer),
    }
  }

  fn transformer_updated(&self, new_transformer: Box<dyn Transformable>) -> Box<dyn Transformable> {
    match new_transformer.is_stopped() {
      true => {
        let mut ret = self.clone();
        ret.stack.pop();
        ret.stack.push(new_transformer);
        ret.stack.push(Box::new(ContinuousTransformer::new(
          self.config(),
          TransformerTypes::Hiragana,
        )));

        Box::new(ret)
      }
      false => self.replace_last_element(new_transformer),
    }
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    return Box::new(Canceled::new(self.config()));
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    match self.send_target() {
      v if v.is_stopped() => Box::new(Stopped::new(self.config(), self.stopped_buffer_content())),
      v => v.push_enter(),
    }
  }
}

impl Displayable for UnknownWord {
  fn buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .fold("".to_string(), |acc, tf| acc + &tf.buffer_content())
  }

  fn display_string(&self) -> String {
    "[登録: ".to_string() + &self.word.display_string() + "]" + &self.buffer_content()
  }
}

impl AsTransformerTrait for UnknownWord {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.clone(),
      None => Box::new(Stopped::empty(self.config())),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tds;
  use crate::tests::{dummy_conf, test_transformer};
  use TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, UnknownWordTransformer, Word::new("みちご", None);
        ["hiragana", "[登録: みちご]ひらがな", UnknownWord],
        // ["Kannji", "[登録: みちご]▽かんじ", UnknownWord],
        // ["Kannji ", "[登録: みちご]▼漢字", UnknownWord],
        // ["Kannji \n","[登録: みちご]漢字", UnknownWord],
        // ["Michi \nGo \n\n","未知語",Stopped]
    ];
    test_transformer(items);
  }
}

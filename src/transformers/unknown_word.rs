use super::{
  AsTransformerTrait, Config, ContinuousTransformer, Displayable, Stackable, StoppedTransformer,
  Transformable, TransformerTypes, WithConfig,
};
use crate::keyboards::{KeyCode, Keyboard};

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

#[derive(Clone)]
pub struct UnknownWordTransformer {
  config: Config,
  word: Word,
  stack: Vec<Box<dyn Transformable>>,
}

impl UnknownWordTransformer {
  pub fn new(config: Config, word: Word) -> Self {
    UnknownWordTransformer {
      config: config.clone(),
      word,
      stack: vec![
        box StoppedTransformer::empty(config.clone()),
        box ContinuousTransformer::new(config, TransformerTypes::Hiragana),
      ],
    }
  }

  fn new_from_stack(&self, stack: Vec<Box<dyn Transformable>>) -> Self {
    let mut ret = self.clone();
    ret.stack = stack;

    ret
  }
}

impl WithConfig for UnknownWordTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for UnknownWordTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::UnknownWord
  }

  fn try_change_transformer(
    &self,
    keyboard: &Box<dyn Keyboard>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    let new_transformer = self
      .send_target()
      .try_change_transformer(keyboard, last_key_code);

    match new_transformer {
      Some(tf) => Some(self.replace_last_element(tf)),
      None => None,
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    if character == '\n' || character == ' ' {
      return self.as_trait();
    }

    self.replace_last_element(self.send_target().push_character(character))
  }

  fn push_space(&self) -> Box<dyn Transformable> {
    self.replace_last_element(self.send_target().push_space())
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    if self.send_target().is_empty() {
      return box StoppedTransformer::completed(self.config(), self.buffer_content());
    }

    self.replace_last_element(self.send_target().push_enter())
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    self.pop().0
  }

  fn push_delete(&self) -> Box<dyn Transformable> {
    self.push_backspace()
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    match self.send_target().transformer_type() {
      TransformerTypes::UnknownWord | TransformerTypes::Henkan => {
        self.replace_last_element(self.send_target().push_escape())
      }
      _ => self.to_canceled(),
    }
  }
}

impl Displayable for UnknownWordTransformer {
  fn buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .fold("".to_string(), |acc, item| acc + &item.buffer_content())
  }

  fn display_string(&self) -> String {
    let buf = self
      .stack
      .iter()
      .fold("".to_string(), |acc, item| acc + &item.display_string());

    "[登録: ".to_string() + &self.word.display_string() + "]" + &buf
  }
}

impl AsTransformerTrait for UnknownWordTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.clone(),
      None => Box::new(StoppedTransformer::empty(self.config())),
    }
  }
}

impl Stackable for UnknownWordTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.new_from_stack(self.stack.clone());

    ret.stack.push(item);

    Box::new(ret)
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();
    if ret.send_target().transformer_type() == TransformerTypes::Continuous
      && ret.send_target().is_empty()
    {
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
        dbg!(ret.as_trait());

        match ret.is_all_stopped() {
          true => ret.push(box ContinuousTransformer::new(
            ret.config(),
            TransformerTypes::Hiragana,
          )),
          false => box ret,
        }
      }
      false => {
        ret.stack.pop();
        ret.stack.push(item);

        match ret.is_all_stopped() {
          true => ret.push(box ContinuousTransformer::new(
            ret.config(),
            TransformerTypes::Hiragana,
          )),
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

    let items = tds![conf, UnknownWordTransformer, Word::new("みちご", None);
      ["[escape]", "", Stopped(Canceled)],
      ["hiragana", "[登録: みちご]ひらがな", UnknownWord],
      ["Kannji", "[登録: みちご]▽かんじ", UnknownWord],
      ["Kannji ", "[登録: みちご]▼漢字", UnknownWord],
      ["Kannji \n", "[登録: みちご]漢字", UnknownWord],
      ["Kannji \n\n", "漢字", Stopped(Compleated)],
      ["Michi \nGo", "[登録: みちご]未知▽ご", UnknownWord],
      ["Michi \nGo ", "[登録: みちご]未知▼語", UnknownWord],
      ["Michi \nGo \n", "[登録: みちご]未知語", UnknownWord],
      ["Michi \nGo \n[backspace]", "[登録: みちご]未知", UnknownWord],
      ["Michi \nGo \n[backspace][backspace]", "[登録: みちご]未", UnknownWord],
      ["Michi \nGo \n\n","未知語", Stopped(Compleated)],
      ["AA", "[登録: みちご]▽あ*あ", UnknownWord],
      ["AAA", "[登録: みちご][登録: ああ]", UnknownWord],
      ["AAAOkuRi", "[登録: みちご][登録: ああ]▼送り", UnknownWord],
      ["AAAOkuRi[escape]", "[登録: みちご][登録: ああ]▽おく", UnknownWord],
      ["AAAOkuRi[escape][escape]", "[登録: みちご][登録: ああ]", UnknownWord],
      ["AAAOkuRi[escape][escape][escape]", "[登録: みちご]▽あ", UnknownWord],
      ["AAAOkuRi[escape][escape][escape][escape]", "[登録: みちご]", UnknownWord],
      ["AAAOkuRi[escape][escape][escape][escape][escape]", "", Stopped(Canceled)],
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

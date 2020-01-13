use super::tables::hiragana_convert;
use super::{
  AsTransformerTrait, BufferState, Config, Displayable, HenkanTransformer, Stackable,
  StoppedTransformer, Transformable, TransformerTypes, WithConfig,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::{set, tf};
use std::collections::HashSet;
use BufferState::*;

#[derive(Clone)]
pub struct HiraganaTransformer {
  config: Config,
  stopped: String,
  buffer: String,
}

impl HiraganaTransformer {
  pub fn new(config: Config) -> Self {
    HiraganaTransformer {
      config,
      stopped: "".to_string(),
      buffer: "".to_string(),
    }
  }

  pub fn from_buffer<S: Into<String>>(config: Config, buffer: S) -> Self {
    let mut ret = Self::new(config);
    ret.buffer = buffer.into();

    ret
  }

  pub fn from_stopped_and_buffer<S: Into<String>>(config: Config, stopped: S, buffer: S) -> Self {
    let mut ret = Self::new(config);
    ret.stopped = stopped.into();
    ret.buffer = buffer.into();

    ret
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![
      TransformerTypes::Direct,
      TransformerTypes::Abbr,
      TransformerTypes::Katakana,
      TransformerTypes::EnKatakana,
      TransformerTypes::EmEisu
    ]
  }

  fn try_enter_henkan(&self, character: char) -> Option<Box<dyn Transformable>> {
    match character.is_uppercase() {
      true => Some(box HenkanTransformer::new(
        self.config(),
        TransformerTypes::Hiragana,
      )),
      false => None,
    }
  }

  fn try_enter_abbr(&self, character: char) -> Option<Box<dyn Transformable>> {
    match character {
      '/' => unimplemented!(),
      _ => None,
    }
  }
}

impl WithConfig for HiraganaTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for HiraganaTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Hiragana
  }

  fn try_change_transformer(
    &self,
    keyboard: &Box<dyn Keyboard>,
    _: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    let transformer_type = self
      .config
      .key_config()
      .try_change_transformer(&Self::allow_transformers(), keyboard.pressing_keys());

    Some(tf!(self.config(), transformer_type?))
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    if let Some(tf) = self.try_enter_abbr(character) {
      return Some(vec![tf]);
    }
    if let Some(tf) = self.try_enter_henkan(character) {
      return tf.push_character(character.to_lowercase().next()?);
    }

    // TODO: 停止したバッファを複数返せるようにする
    hiragana_convert(&self.buffer, character)
      .map(|vec| match &*vec {
        [] => vec![],
        [pair] if !pair.is_stopped() => {
          vec![
            box Self::from_stopped_and_buffer(self.config(), &self.stopped, &pair.to_string())
              as Box<dyn Transformable>,
          ]
        }
        [pair] => vec![box StoppedTransformer::completed(
          self.config(),
          self.stopped.clone() + &pair.to_string(),
        ) as Box<dyn Transformable>],
        vec => {
          let (last, elems) = vec.split_last().unwrap();
          let stopped = elems
            .iter()
            .fold("".to_string(), |acc, pair| acc + &pair.to_string());

          match last.state() {
            Continue => {
              vec![
                box Self::from_stopped_and_buffer(self.config(), stopped, last.to_string())
                  as Box<dyn Transformable>,
              ]
            }
            Stop => {
              vec![
                box StoppedTransformer::completed(self.config(), stopped + &last.to_string())
                  as Box<dyn Transformable>,
              ]
            }
          }
        }
      })
      .or(Some(vec![]))
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    match self.is_empty() {
      true => Some(vec![]),
      false => Some(vec![self.pop().0]),
    }
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    match self.is_empty() {
      true => None,
      false => {
        let tf = self.pop().0;
        match tf.is_canceled() {
          true => Some(vec![]),
          false => Some(vec![tf]),
        }
      }
    }
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.push_backspace()
  }
}

impl Displayable for HiraganaTransformer {
  fn buffer_content(&self) -> String {
    self.stopped.clone() + &self.buffer
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }
}

impl AsTransformerTrait for HiraganaTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

impl Stackable for HiraganaTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!()
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();
    ret.buffer.pop();

    let ret = match (ret.buffer.len() == 0, &self.stopped.clone() as &str) {
      (true, "") => ret.to_canceled(),
      (false, "") => box Self::from_buffer(self.config(), &ret.buffer),
      (true, _) => ret.to_completed(),
      (false, s) => box Self::from_stopped_and_buffer(self.config(), s, &ret.buffer),
    };

    (ret, Some(self.to_canceled()))
  }

  fn replace_last_element(&self, _: Vec<Box<dyn Transformable>>) -> Vec<Box<dyn Transformable>> {
    vec![box self.clone()]
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    vec![]
  }

  fn child_transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Hiragana
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;
  use crate::{tds, tfe};

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, Hiragana;
      ["a", "あ", Stopped(Compleated)],
      ["k", "k", Hiragana],
      ["k[escape]", "", Stopped(Canceled)],
      ["k[backspace]", "", Stopped(Canceled)],
      ["ts[backspace]", "t", Hiragana],
      ["ka", "か", Stopped(Compleated)],
      ["tt", "っt", Hiragana],
      ["tt[backspace]", "っ", Stopped(Compleated)],
      ["tte", "って", Stopped(Compleated)],
      ["tte[backspace]", "っ", Stopped(Compleated)],
      ["k[escape]", "", Stopped(Canceled)],
      ["Kannji", "▽かんじ", Henkan],
      ["Kanji", "▽かんじ", Henkan],
    ];
    test_transformer(items);
  }

  #[test]
  fn stack() {
    let conf = dummy_conf();

    let tf = tfe!(conf, Hiragana; "").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = tfe!(conf, Hiragana; "ts").pop().0;
    assert_eq!(tf.transformer_type(), Hiragana);
    assert_eq!(tf.buffer_content(), "t");
  }
}

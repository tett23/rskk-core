use std::collections::HashSet;
use std::rc::Rc;

use super::tables::{BufferPairs, LetterType};
use super::{
  AbbrTransformer, AsTransformerTrait, Displayable, HenkanTransformer, Stackable, Transformable,
  TransformerTypes, WithContext,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::{set, tf, Context};

#[derive(Clone)]
pub struct HiraganaTransformer {
  context: Rc<Context>,
  buffer: BufferPairs,
}

impl HiraganaTransformer {
  pub fn new(context: Rc<Context>) -> Self {
    HiraganaTransformer {
      context,
      buffer: BufferPairs::new(LetterType::Hiragana),
    }
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![
      TransformerTypes::Direct,
      TransformerTypes::Katakana,
      TransformerTypes::EnKatakana,
      TransformerTypes::EmEisu
    ]
  }

  fn try_enter_henkan(&self, character: char) -> Option<Box<dyn Transformable>> {
    match character.is_uppercase() {
      true => Some(box HenkanTransformer::new(
        self.clone_context(),
        TransformerTypes::Hiragana,
      )),
      false => None,
    }
  }

  fn try_enter_abbr(&self, character: char) -> Option<Box<dyn Transformable>> {
    match character {
      '/' => Some(box AbbrTransformer::new(self.clone_context())),
      _ => None,
    }
  }
}

impl WithContext for HiraganaTransformer {
  fn context(&self) -> &Context {
    &self.context
  }

  fn clone_context(&self) -> Rc<Context> {
    self.context.clone()
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
      .context
      .config()
      .key_config()
      .try_change_transformer(&Self::allow_transformers(), keyboard.pressing_keys());

    Some(tf!(self.clone_context(), transformer_type?))
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    if let Some(tf) = self.try_enter_abbr(character) {
      return Some(vec![tf]);
    }
    if let Some(tf) = self.try_enter_henkan(character) {
      return tf.push_character(character.to_lowercase().next()?);
    }

    let mut tf = self.clone();
    tf.buffer.push(character);
    // TODO: 停止したバッファを複数返せるようにする
    Some(vec![match tf.buffer.is_stopped() {
      true => tf.to_completed(),
      false => box tf,
    }])
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    match self.buffer.is_empty() {
      true => None,
      false => Some(vec![]),
    }
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    match self.buffer.is_empty() {
      true => None,
      false => {
        let mut tf = self.clone();
        tf.buffer.remove_last();

        match tf.buffer.is_empty() {
          true => Some(vec![]),
          false if tf.buffer.is_stopped() => Some(vec![tf.to_completed()]),
          false => Some(vec![box tf]),
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
    self.buffer.to_string()
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }
}

impl AsTransformerTrait for HiraganaTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }
}

impl Stackable for HiraganaTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!()
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut tf = self.clone();
    let tf = match tf.buffer.is_empty() {
      true => tf.to_canceled(),
      false => {
        tf.buffer.remove_last();
        box tf
      }
    };

    (tf, None)
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
  use crate::tests::{dummy_context, test_transformer};
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;
  use crate::{tds, tfe};

  #[test]
  fn it_works() {
    let conf = dummy_context();

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
    let conf = dummy_context();

    let tf = tfe!(conf, Hiragana; "").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = tfe!(conf, Hiragana; "ts").pop().0;
    assert_eq!(tf.transformer_type(), Hiragana);
    assert_eq!(tf.buffer_content(), "t");
  }
}

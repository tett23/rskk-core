use std::rc::Rc;

use super::tables::{BufferPairs, LetterType};
use super::{
  AbbrTransformer, AsTransformerTrait, Displayable, HenkanTransformer, Stackable, Transformable,
  TransformerTypes, WithContext,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::{set, tf, Context};
use std::collections::HashSet;

#[derive(Clone)]
pub struct KatakanaTransformer {
  context: Rc<Context>,
  buffer: BufferPairs,
}

impl KatakanaTransformer {
  pub fn new(context: Rc<Context>) -> Self {
    KatakanaTransformer {
      context,
      buffer: BufferPairs::new(LetterType::Katakana),
    }
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![
      TransformerTypes::Direct,
      TransformerTypes::Hiragana,
      TransformerTypes::EnKatakana,
      TransformerTypes::EmEisu
    ]
  }

  fn try_enter_henkan(&self, character: char) -> Option<Box<dyn Transformable>> {
    match character.is_uppercase() {
      true => Some(box HenkanTransformer::new(
        self.clone_context(),
        TransformerTypes::Katakana,
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

impl WithContext for KatakanaTransformer {
  fn context(&self) -> &Context {
    &self.context
  }

  fn clone_context(&self) -> Rc<Context> {
    self.context.clone()
  }
}

impl Transformable for KatakanaTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Katakana
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

impl Displayable for KatakanaTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.to_string()
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }
}

impl AsTransformerTrait for KatakanaTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }
}

impl Stackable for KatakanaTransformer {
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
    TransformerTypes::Katakana
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

    let items = tds![conf, Katakana;
      ["a", "ア", Stopped(Compleated)],
      ["k", "k", Katakana],
      ["k[escape]", "", Stopped(Canceled)],
      ["k[backspace]", "", Stopped(Canceled)],
      ["ts[backspace]", "t", Katakana],
      ["ka", "カ", Stopped(Compleated)],
      ["tt", "ッt", Katakana],
      ["tt[backspace]", "ッ", Stopped(Compleated)],
      ["tte", "ッテ", Stopped(Compleated)],
      ["tte[backspace]", "ッ", Stopped(Compleated)],
      ["k[escape]", "", Stopped(Canceled)],
      ["Kannji", "▽カンジ", Henkan],
      ["Kanji", "▽カンジ", Henkan],
    ];
    test_transformer(items);
  }

  #[test]
  fn stack() {
    let conf = dummy_context();

    let tf = tfe!(conf, Katakana; "").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = tfe!(conf, Katakana; "ts").pop().0;
    assert_eq!(tf.transformer_type(), Katakana);
    assert_eq!(tf.buffer_content(), "t");
  }
}

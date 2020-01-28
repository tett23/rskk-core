use std::cell::RefCell;
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
  context: Rc<RefCell<Context>>,
  buffer: BufferPairs,
}

impl HiraganaTransformer {
  pub fn new(context: Rc<RefCell<Context>>) -> Self {
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
  fn clone_context(&self) -> Rc<RefCell<Context>> {
    self.context.clone()
  }

  #[cfg(test)]
  fn set_context(&mut self, context: Rc<RefCell<Context>>) {
    self.context = context;
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
      .borrow()
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
    Some(vec![match tf.buffer.is_stopped() {
      true => tf.to_completed_with_update_buffer(tf.buffer.to_string()),
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
  use crate::tests::dummy_context;
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_context();

    let vec = crate::tds![conf, Hiragana;
      ["a", { display: "", stopped_buffer: "あ", transformer_type: Stopped(Compleated) }],
      ["k", { display: "k", transformer_type: Hiragana }],
      ["k[escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["k[backspace]", { display: "", transformer_type: Stopped(Canceled) }],
      ["ts[backspace]", { display: "t", transformer_type: Hiragana }],
      ["ka", { display: "", stopped_buffer: "か", transformer_type: Stopped(Compleated) }],
      ["tt", { display: "t", stopped_buffer: "っ", transformer_type: Hiragana }],
      ["tt[backspace]", { display: "", stopped_buffer: "っ", transformer_type: Stopped(Compleated) }],
      ["tte", { display: "", stopped_buffer: "って", transformer_type: Stopped(Compleated) }],
      ["Kannji", { display: "▽かんじ", transformer_type: Henkan }],
      ["Kanji", { display: "▽かんじ", transformer_type: Henkan }],
    ];
    crate::tests::helpers::TestData::batch(vec);
  }

  #[test]
  fn stack() {
    let conf = dummy_context();

    let tf = crate::tfe!(conf, Hiragana; "").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = crate::tfe!(conf, Hiragana; "ts").pop().0;
    assert_eq!(tf.transformer_type(), Hiragana);
    assert_eq!(tf.buffer_content(), "t");
  }
}

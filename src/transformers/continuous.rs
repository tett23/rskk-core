use std::cell::RefCell;
use std::rc::Rc;

use super::{
  AsTransformerTrait, Displayable, Stackable, Transformable, TransformerTypes, WithContext,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::{tf, Context};

#[derive(Clone)]
pub struct ContinuousTransformer {
  context: Rc<RefCell<Context>>,
  current_transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
  buffer: String,
}

impl ContinuousTransformer {
  pub fn new(context: Rc<RefCell<Context>>, transformer_type: TransformerTypes) -> Self {
    ContinuousTransformer {
      context,
      current_transformer_type: transformer_type,
      stack: vec![],
      buffer: String::new(),
    }
  }

  fn base_transformer(&self) -> Box<dyn Transformable> {
    tf!(self.clone_context(), self.current_transformer_type)
  }

  fn push_new_base_transformer(&mut self) {
    self.stack.push(self.base_transformer())
  }
}

impl WithContext for ContinuousTransformer {
  fn clone_context(&self) -> Rc<RefCell<Context>> {
    self.context.clone()
  }

  #[cfg(test)]
  fn set_context(&mut self, context: Rc<RefCell<Context>>) {
    self.context = context;
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
    let mut tf = self.clone();
    match &*tf.stack {
      [] => tf.push_new_base_transformer(),
      [.., last] if last.is_stopped() => tf.push_new_base_transformer(),
      _ => {}
    }

    tf.stack
      .last()
      .and_then(|last| Some(tf.replace_last_element(last.push_character(character)?)))
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    if self.stack.is_empty() {
      return Some(vec![]);
    }

    Some(self.replace_last_element(self.stack.last()?.push_escape()?))
  }

  fn push_enter(&self) -> Option<Vec<Box<dyn Transformable>>> {
    match &*self.stack {
      [] => Some(vec![]),
      [.., last] if last.is_stopped() => Some(vec![self.to_completed()]),
      [.., last] => Some(self.replace_last_element(last.push_enter()?)),
    }
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    match &*self.stack {
      [] => Some(vec![]),
      [.., last] => Some(self.replace_last_element(last.push_backspace()?)),
    }
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
  use crate::tests::dummy_context;
  use crate::transformers::StoppedReason;
  use StoppedReason::*;
  use TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_context();

    let vec = crate::tds![conf, ContinuousTransformer, Hiragana;
      ["[escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["[backspace]", { display: "", transformer_type: Stopped(Canceled) }],
      ["aa[backspace]", { display: "あ", transformer_type: Continuous }],
      ["ak[backspace]", { display: "あ", transformer_type: Continuous }],
      ["aa[backspace]i", { display: "あい", transformer_type: Continuous }],
      ["ak\n", { display: "あ", transformer_type: Continuous }],
      ["hiragana", { display: "ひらがな", transformer_type: Continuous }],
      ["hiragana\n", { stopped_buffer: "ひらがな", transformer_type: Stopped(Compleated) }],
      ["Kannji", { display: "▽かんじ", transformer_type: Continuous }],
      ["Kannji \n", { display: "漢字", transformer_type: Continuous }],
      ["Kannji \n\n", { stopped_buffer: "漢字", transformer_type: Stopped(Compleated) }],
      ["Kannji \n[backspace]", { display: "漢", transformer_type: Continuous }],
      ["Kannji \n[backspace]a", { display: "漢あ", transformer_type: Continuous }],
    ];
    crate::tests::helpers::TestData::batch(vec);
  }
}

use std::cell::RefCell;
use std::rc::Rc;

use super::{
  AsTransformerTrait, ContinuousTransformer, Displayable, KeyCode, SelectCandidateTransformer,
  Stackable, StoppedTransformer, Transformable, TransformerTypes, UnknownWordTransformer,
  WithContext, Word,
};
use crate::Context;

#[derive(Clone)]
pub struct AbbrTransformer {
  context: Rc<RefCell<Context>>,
  stack: Vec<Box<dyn Transformable>>,
}

impl AbbrTransformer {
  pub fn new(context: Rc<RefCell<Context>>) -> Self {
    Self {
      context: context.clone(),
      stack: vec![box ContinuousTransformer::new(
        context,
        TransformerTypes::Direct,
      )],
    }
  }

  fn try_composition(&self) -> Box<dyn Transformable> {
    self
      .try_transition_to_select_candidate()
      .map(|tf| -> Box<dyn Transformable> { box tf })
      .unwrap_or(box self.transition_to_unknown_word())
  }

  fn try_transition_to_select_candidate(&self) -> Option<SelectCandidateTransformer> {
    self
      .context
      .borrow()
      .dictionary()
      .transform(self.to_word().to_dic_read()?)
      .map(|dic_entry| {
        SelectCandidateTransformer::new(self.clone_context(), dic_entry, self.to_word())
      })
  }

  fn transition_to_unknown_word(&self) -> UnknownWordTransformer {
    UnknownWordTransformer::new(self.clone_context(), { self.to_word() })
  }

  fn to_word(&self) -> Word {
    Word::new_abbr(self.buffer_content())
  }

  fn clear_stack(&mut self) {
    self.stack = vec![box ContinuousTransformer::new(
      self.clone_context(),
      TransformerTypes::Direct,
    )]
  }
}

impl WithContext for AbbrTransformer {
  fn clone_context(&self) -> Rc<RefCell<Context>> {
    self.context.clone()
  }

  #[cfg(test)]
  fn set_context(&mut self, context: Rc<RefCell<Context>>) {
    self.context = context;
  }
}

impl Transformable for AbbrTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Abbr
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.stack.last()?.push_character(character)?))
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.stack.last()?.push_escape()?))
  }

  fn push_enter(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let tfs = self.stack.last()?.push_enter()?;
    match &*tfs {
      [] => Some(vec![]),
      [last] if last.is_stopped() => Some(vec![last.clone()]),
      _ => Some(self.replace_last_element(tfs)),
    }
  }

  fn push_space(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let mut tf = self.clone();
    match &*tf.stack {
      [] => Some(vec![]),
      [first] if first.transformer_type() == TransformerTypes::Continuous && first.is_empty() => {
        Some(vec![])
      }
      [first] if first.transformer_type() == TransformerTypes::Continuous => {
        tf.stack.push(tf.try_composition());
        Some(vec![box tf])
      }
      [.., last] => Some(tf.replace_last_element(last.push_space()?)),
    }
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    if self.stack.len() == 1 && self.is_empty() {
      return Some(vec![]);
    }

    let tf = self.replace_last_element(self.send_target().push_delete()?);
    if !tf.is_empty() {
      return Some(tf);
    }

    let mut tf = self.clone();
    tf.clear_stack();

    Some(vec![box tf])
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.push_delete()
  }

  fn push_any_character(&self, key_code: &KeyCode) -> Option<Vec<Box<dyn Transformable>>> {
    let tfs = self.stack.last()?.push_any_character(key_code)?;
    match &*tfs {
      [] => Some(vec![]),
      [.., last] if last.is_stopped() => Some(vec![last.clone()]),
      _ => Some(self.replace_last_element(tfs)),
    }
  }
}

impl Displayable for AbbrTransformer {
  fn buffer_content(&self) -> String {
    self.send_target().buffer_content()
  }

  fn display_string(&self) -> String {
    match &*self.stack {
      [tf] if tf.transformer_type() == TransformerTypes::Continuous => {
        "▽".to_owned() + &tf.display_string()
      }
      [.., last] => last.display_string(),
      _ => "".to_owned(),
    }
  }
}

impl AsTransformerTrait for AbbrTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.clone(),
      None => box StoppedTransformer::completed(self.clone_context()),
    }
  }
}

impl Stackable for AbbrTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.stack.push(item);

    box ret
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();

    let item = ret.stack.pop();
    if ret.stack.len() == 0 {
      return (self.to_canceled(), item);
    }

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
}

#[cfg(test)]
mod tests {
  use crate::tests::dummy_context;
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_context();

    let vec = crate::tds![conf, Abbr;
      ["[backspace]", { display: "", transformer_type: Stopped(Canceled) }],
      ["[escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["a[backspace]", { display: "▽", transformer_type: Abbr }],
      ["test", { display: "▽test", transformer_type: Abbr }],
      ["test\n", { display: "", stopped_buffer: "test", transformer_type: Stopped(Compleated) }],
      ["hoge ", { display: "[登録: hoge]", transformer_type: Abbr }],
      ["hoge [escape]", { display: "▽hoge", transformer_type: Abbr }],
      ["hoge [backspace]", { display: "[登録: hoge]", transformer_type: Abbr }],
      ["hoge fuga", { display: "[登録: hoge]ふが", transformer_type: Abbr }],
      ["hoge fuga\n", { display: "", stopped_buffer: "ふが", transformer_type: Stopped(Compleated) }],
    ];
    crate::tests::helpers::TestData::batch(vec);
  }
}

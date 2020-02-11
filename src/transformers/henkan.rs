use std::cell::RefCell;
use std::rc::Rc;

use super::{
  AsTransformerTrait, Displayable, KeyCode, Stackable, Transformable, TransformerTypes,
  WithContext, YomiTransformer,
};
use crate::Context;

#[derive(Clone)]
pub struct HenkanTransformer {
  context: Rc<RefCell<Context>>,
  current_transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
}

impl HenkanTransformer {
  pub fn new(context: Rc<RefCell<Context>>, transformer_type: TransformerTypes) -> Self {
    HenkanTransformer {
      context: context.clone(),
      current_transformer_type: transformer_type,
      stack: vec![box YomiTransformer::new(context, transformer_type)],
    }
  }
}

impl WithContext for HenkanTransformer {
  fn clone_context(&self) -> Rc<RefCell<Context>> {
    self.context.clone()
  }

  fn set_context(&mut self, context: Rc<RefCell<Context>>) {
    self.context = context;
  }
}

impl Transformable for HenkanTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Henkan
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.stack.last()?.push_character(character)?))
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    dbg!(&self.stack);
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
    let mut new_tf = self.send_target().push_space()?;
    let vec = match self.send_target().transformer_type() {
      TransformerTypes::Yomi => {
        let mut tf = self.clone();
        tf.stack.append(&mut new_tf);

        return Some(vec![box tf]);
      }
      _ => new_tf,
    };

    Some(self.replace_last_element(vec))
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.send_target().push_delete()?))
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.send_target().push_backspace()?))
  }

  fn push_any_character(&self, key_code: &KeyCode) -> Option<Vec<Box<dyn Transformable>>> {
    let tfs = self.stack.last()?.push_any_character(key_code)?;
    match &*tfs {
      [] => Some(vec![]),
      [last] if last.is_stopped() => Some(vec![last.clone()]),
      _ => Some(self.replace_last_element(tfs)),
    }
  }
}

impl Displayable for HenkanTransformer {
  fn buffer_content(&self) -> String {
    self.send_target().buffer_content()
  }

  fn display_string(&self) -> String {
    self.send_target().display_string()
  }
}

impl AsTransformerTrait for HenkanTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.clone(),
      None => self.to_completed(),
    }
  }
}

impl Stackable for HenkanTransformer {
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

    let vec = crate::tds![conf, HenkanTransformer, Hiragana;
      ["hiragana", { display: "▽ひらがな", transformer_type: Henkan }],
      ["hiragana\n", { stopped_buffer: "ひらがな", transformer_type: Stopped(Compleated) }],
      ["hiragana[escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["kannji ", { display: "▼漢字", transformer_type: Henkan }],
      ["kannji [backspace]", { display: "▽かんじ", transformer_type: Henkan }],
      ["kannji \n", { stopped_buffer: "漢字", transformer_type: Stopped(Compleated) }],
      ["okuR", { display: "▽おく*r", transformer_type: Henkan }],
      ["okuR\n", { stopped_buffer: "おく", transformer_type: Stopped(Compleated) }],
      ["okuR[escape]", { display: "▽おく", transformer_type: Henkan }],
      ["okuRi", { display: "▼送り", transformer_type: Henkan }],
      ["okuRi[escape]", { display: "▽おく", transformer_type: Henkan }],
      ["okuRi\n", { stopped_buffer: "送り", transformer_type: Stopped(Compleated) }],
      ["michigo ", { display: "[登録: みちご]", transformer_type: Henkan }],
      ["aA", { display: "[登録: あ*あ]", transformer_type: Henkan }],
      ["michigo [backspace]", { display: "[登録: みちご]", transformer_type: Henkan }],
      ["aa[backspace]", { display: "▽あ", transformer_type: Henkan }],
      ["aa[backspace][backspace]", { display: "▽", transformer_type: Henkan }],
      ["aa[backspace][backspace][backspace]", { display: "", transformer_type: Stopped(Canceled) }],
      ["aA", { display: "[登録: あ*あ]", transformer_type: Henkan }],
      ["aA[escape]", { display: "▽あ", transformer_type: Henkan }],
      ["aKa", { display: "[登録: あ*か]", transformer_type: Henkan }],
      ["aKa[escape]", { display: "▽あ", transformer_type: Henkan }],
      ["aTte", { display: "[登録: あ*って]", transformer_type: Henkan }],
      ["aTte[escape]", { display: "▽あ", transformer_type: Henkan }],
      ["aTsu", { display: "[登録: あ*つ]", transformer_type: Henkan }],
      ["aTsu[escape]", { display: "▽あ", transformer_type: Henkan }],
    ];
    crate::tests::helpers::TestData::batch(vec);

    // TODO: カタカナ時のテスト
  }
}

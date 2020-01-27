use std::rc::Rc;

use super::{
  AsTransformerTrait, ContinuousTransformer, Displayable, Stackable, Transformable,
  TransformerTypes, WithContext, Word,
};
use crate::Context;

#[derive(Clone)]
pub struct UnknownWordTransformer {
  context: Rc<Context>,
  word: Word,
  stack: Vec<Box<dyn Transformable>>,
}

impl UnknownWordTransformer {
  pub fn new(context: Rc<Context>, word: Word) -> Self {
    UnknownWordTransformer {
      context,
      word,
      stack: vec![],
    }
  }

  fn new_from_stack(&self, stack: Vec<Box<dyn Transformable>>) -> Self {
    let mut ret = self.clone();
    ret.stack = stack;

    ret
  }
}

impl WithContext for UnknownWordTransformer {
  fn context(&self) -> &Context {
    &self.context
  }

  fn clone_context(&self) -> Rc<Context> {
    self.context.clone()
  }
}

impl Transformable for UnknownWordTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::UnknownWord
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.send_target().push_character(character)?))
  }

  fn push_space(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.send_target().push_space()?))
  }

  fn push_enter(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let tfs = self.send_target().push_enter()?;
    if tfs.last()?.is_compleated() {
      return Some(vec![tfs.last()?.clone()]);
    }

    Some(self.replace_last_element(tfs))
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    if self.is_empty() {
      return Some(vec![box self.clone()]);
    }

    Some(self.replace_last_element(self.stack.last()?.push_backspace()?))
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.push_backspace()
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    if self.stack.is_empty() {
      return Some(vec![]);
    }

    Some(self.replace_last_element(self.stack.last()?.push_escape()?))
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

    dbg!(self.word.display_string());
    "[登録: ".to_string() + &self.word.display_string() + "]" + &buf
  }
}

impl AsTransformerTrait for UnknownWordTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    self
      .stack
      .last()
      .map(|tf| tf.clone())
      .unwrap_or(box ContinuousTransformer::new(
        self.clone_context(),
        TransformerTypes::Hiragana,
      ))
  }
}

impl Stackable for UnknownWordTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.new_from_stack(self.stack.clone());

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
  use super::super::tables::LetterType;
  use super::*;
  use crate::tds;
  use crate::tests::{dummy_context, test_transformer};
  use crate::transformers::StoppedReason;
  use StoppedReason::*;
  use TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_context();

    let items = tds![conf, UnknownWordTransformer, Word::from((LetterType::Hiragana, "michigo"));
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
      ["AK", "[登録: みちご]▽あ*k", UnknownWord],
      ["AA", "[登録: みちご][登録: あ*あ]", UnknownWord],
      ["AAA", "[登録: みちご][登録: あ*あ]▽あ", UnknownWord],
      ["AAOkuRi", "[登録: みちご][登録: あ*あ]▼送り", UnknownWord],
      ["AAOkuRi[escape]", "[登録: みちご][登録: あ*あ]▽おく", UnknownWord],
      ["AAOkuRi[escape][escape]", "[登録: みちご][登録: あ*あ]", UnknownWord],
      ["AAOkuRi[escape][escape][escape]", "[登録: みちご]▽あ", UnknownWord],
      ["AAOkuRi[escape][escape][escape][escape]", "[登録: みちご]", UnknownWord],
      ["AAOkuRi[escape][escape][escape][escape][escape]", "", Stopped(Canceled)],
    ];
    test_transformer(items);
  }
}

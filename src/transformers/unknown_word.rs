use super::{
  AsTransformerTrait, ContinuousTransformer, Displayable, Stackable, Transformable,
  TransformerTypes, WithContext, Word,
};
use crate::Context;

#[derive(Clone)]
pub struct UnknownWordTransformer {
  context: Context,
  word: Word,
  stack: Vec<Box<dyn Transformable>>,
}

impl UnknownWordTransformer {
  pub fn new(context: Context, word: Word) -> Self {
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
  fn clone_context(&self) -> Context {
    self.context.clone()
  }

  fn context(&self) -> &Context {
    &self.context
  }

  fn set_context(&mut self, context: Context) {
    self.context = context;
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
    match self.stack.last() {
      None => Some(vec![]),
      Some(tf)
        if tf.transformer_type() == TransformerTypes::Continuous
          && tf.child_transformer_type() != TransformerTypes::Henkan
          && tf.is_empty() =>
      {
        Some(vec![])
      }
      Some(tf) => Some(self.replace_last_element(tf.push_escape()?)),
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
  use crate::tests::dummy_context;
  use crate::transformers::StoppedReason;
  use StoppedReason::*;
  use TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_context();
    let word = Word::from((LetterType::Hiragana, "michigo"));

    let vec = crate::tds![conf, UnknownWordTransformer, word;
      ["[escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["hiragana", { display: "[登録: みちご]ひらがな", transformer_type: UnknownWord }],
      ["Kannji", { display: "[登録: みちご]▽かんじ", transformer_type: UnknownWord }],
      ["Kannji ", { display: "[登録: みちご]▼漢字", transformer_type: UnknownWord }],
      ["Kannji \n", { display: "[登録: みちご]漢字", transformer_type: UnknownWord }],
      ["Kannji \n\n", { stopped_buffer: "漢字", transformer_type: Stopped(Compleated) }],
      ["Michi \nGo", { display: "[登録: みちご]未知▽ご", transformer_type: UnknownWord }],
      ["Michi \nGo ", { display: "[登録: みちご]未知▼語", transformer_type: UnknownWord }],
      ["Michi \nGo \n", { display: "[登録: みちご]未知語", transformer_type: UnknownWord }],
      ["Michi \nGo \n[backspace]", { display: "[登録: みちご]未知", transformer_type: UnknownWord }],
      ["Michi \nGo \n[backspace][backspace]", { display: "[登録: みちご]未", transformer_type: UnknownWord }],
      ["Michi \nGo \n\n", { stopped_buffer: "未知語", transformer_type: Stopped(Compleated) }],
      ["AK", { display: "[登録: みちご]▽あ*k", transformer_type: UnknownWord }],
      ["AA", { display: "[登録: みちご][登録: あ*あ]", transformer_type: UnknownWord }],
      ["AAA", { display: "[登録: みちご][登録: あ*あ]▽あ", transformer_type: UnknownWord }],
      ["AAOkuRi", { display: "[登録: みちご][登録: あ*あ]▼送り", transformer_type: UnknownWord }],
      ["AAOkuRi[escape]", { display: "[登録: みちご][登録: あ*あ]▽おく", transformer_type: UnknownWord }],
      ["AAOkuRi[escape][escape]", { display: "[登録: みちご][登録: あ*あ]", transformer_type: UnknownWord }],
      ["AAOkuRi[escape][escape][escape]", { display: "[登録: みちご]▽あ", transformer_type: UnknownWord }],
      ["AAOkuRi[escape][escape][escape][escape]", { display: "[登録: みちご]", transformer_type: UnknownWord }],
      ["AAOkuRi[escape][escape][escape][escape][escape]", { display: "", transformer_type: Stopped(Canceled) }],
    ];
    crate::tests::helpers::TestData::batch(vec);
  }
}

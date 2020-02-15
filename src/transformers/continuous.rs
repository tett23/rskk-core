use super::{
  AsTransformerTrait, Displayable, Stackable, Transformable, TransformerTypes, WithContext,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::{tf, Context, DictionaryEntry};

#[derive(Clone)]
pub struct ContinuousTransformer {
  context: Context,
  current_transformer_type: TransformerTypes,
  child: Box<dyn Transformable>,
  buffer: String,
}

impl ContinuousTransformer {
  pub fn new(context: Context, transformer_type: TransformerTypes) -> Self {
    ContinuousTransformer {
      context: context.clone(),
      current_transformer_type: transformer_type,
      child: tf!(context.new_empty(), transformer_type),
      buffer: String::new(),
    }
  }

  fn new_child_transformer(&self) -> Box<dyn Transformable> {
    tf!(self.new_context(), self.current_transformer_type)
  }

  fn merge_transform_results(&self, tfs: &Vec<Box<dyn Transformable>>) -> ContinuousTransformer {
    self.merge_buffer(tfs).merge_dictionary_updates(tfs)
  }

  fn merge_buffer(&self, tfs: &Vec<Box<dyn Transformable>>) -> ContinuousTransformer {
    match Self::collect_stopped_buffer(tfs) {
      None => self.clone(),
      Some(string) => {
        let mut ret = self.clone();
        ret.buffer.push_str(&string);

        ret
      }
    }
  }

  fn collect_stopped_buffer(tfs: &Vec<Box<dyn Transformable>>) -> Option<String> {
    let ret = tfs
      .iter()
      .map(|tf| tf.context().result().stopped_buffer())
      .filter(|item| item.is_some())
      .map(|item| item.unwrap())
      .fold(String::new(), |acc, item| acc + &item);

    match ret.is_empty() {
      true => None,
      false => Some(ret),
    }
  }

  fn merge_dictionary_updates(&self, tfs: &Vec<Box<dyn Transformable>>) -> ContinuousTransformer {
    match Self::collect_dictonary_updates(tfs) {
      None => self.clone(),
      Some(vec) => {
        let mut ret = self.clone();
        ret.push_dictionary_updates(&vec);

        ret
      }
    }
  }

  fn collect_dictonary_updates(tfs: &Vec<Box<dyn Transformable>>) -> Option<Vec<DictionaryEntry>> {
    let ret = tfs
      .iter()
      .map(|tf| {
        tf.context()
          .result()
          .dictionary_updates()
          .iter()
          .map(|a| a.clone())
          .collect::<Vec<_>>()
      })
      .fold(vec![], |mut acc, mut vec| {
        acc.append(&mut vec);

        acc
      });

    match ret.is_empty() {
      true => None,
      false => Some(ret),
    }
  }
}

impl WithContext for ContinuousTransformer {
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

impl Transformable for ContinuousTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Continuous
  }

  fn try_change_transformer(
    &self,
    keyboard: &Box<dyn Keyboard>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    self.child.try_change_transformer(keyboard, last_key_code)
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    let tfs = self.child.push_character(character)?;
    let tf = self.merge_transform_results(&tfs);

    let mut new_tf = tfs.last()?.clone();
    new_tf.set_context(new_tf.clear_stopped_buffer());

    Some(tf.replace_last_element(vec![new_tf]))
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let tf = self.child.push_escape()?;
    match tf.is_empty() {
      true => Some(vec![]),
      false => Some(self.replace_last_element(tf)),
    }
  }

  fn push_enter(&self) -> Option<Vec<Box<dyn Transformable>>> {
    match self.child.is_empty() {
      true => Some(vec![
        self.to_completed_with_update_buffer(self.buffer.clone())
      ]),
      false => {
        let tfs = self.child.push_enter()?;
        let tf = self.merge_transform_results(&tfs);

        Some(tf.replace_last_element(vec![tf.new_child_transformer()]))
      }
    }
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    match self.child.is_empty() {
      true => {
        let mut tf = self.clone();

        match tf.buffer.pop().is_none() {
          true => Some(vec![]),
          false => Some(vec![box tf]),
        }
      }
      false => Some(self.replace_last_element(self.child.push_backspace()?)),
    }
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.push_backspace()
  }

  fn push_space(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.child.push_space()?))
  }
}

impl Displayable for ContinuousTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.clone() + &self.child.buffer_content()
  }

  fn display_string(&self) -> String {
    self.buffer.clone() + &self.child.display_string()
  }
}

impl AsTransformerTrait for ContinuousTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    self.child.clone()
  }
}

impl Stackable for ContinuousTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!();
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    unreachable!();
  }

  fn replace_last_element(
    &self,
    items: Vec<Box<dyn Transformable>>,
  ) -> Vec<Box<dyn Transformable>> {
    let mut ret = self.clone();

    match items.last() {
      None => {
        ret.child = self.new_child_transformer();

        vec![box ret]
      }
      Some(tf) => {
        ret.child = match tf.is_stopped() {
          true => self.new_child_transformer(),
          false => tf.clone(),
        };
        vec![box ret]
      }
    }
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    vec![self.child.clone()]
  }

  fn child_transformer_type(&self) -> TransformerTypes {
    self.child.transformer_type()
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
      ["[backspace]", { display: "", transformer_type: Stopped(Canceled) }],
      ["a", { display: "あ", transformer_type: Continuous }],
      ["a\n", { display: "", stopped_buffer: "あ", transformer_type: Stopped(Compleated) }],
      ["aa", { display: "ああ", transformer_type: Continuous }],
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

use super::{
  AsTransformerTrait, Displayable, KeyCode, Stackable, Transformable, TransformerTypes,
  WithContext, YomiTransformer,
};
use crate::{tf, Context};

#[derive(Clone)]
pub struct HenkanTransformer {
  context: Context,
  current_transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
}

impl HenkanTransformer {
  pub fn new(context: Context, transformer_type: TransformerTypes) -> Self {
    HenkanTransformer {
      context: context.clone(),
      current_transformer_type: transformer_type,
      stack: vec![box YomiTransformer::new(
        context.new_empty(),
        transformer_type,
      )],
    }
  }
}

impl WithContext for HenkanTransformer {
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

impl Transformable for HenkanTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Henkan
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
    Some(self.replace_last_element(self.stack.last()?.push_space()?))
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.stack.last()?.push_delete()?))
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(self.replace_last_element(self.stack.last()?.push_backspace()?))
  }

  fn push_any_character(&self, key_code: &KeyCode) -> Option<Vec<Box<dyn Transformable>>> {
    let tfs = self.stack.last()?.push_any_character(key_code)?;

    match &*tfs {
      [] => Some(vec![]),
      [last] if last.is_stopped() => {
        let tf = tf!(self.new_context(), self.current_transformer_type);
        let mut tf = key_code
          .printable_key()
          .and_then(|character| Some(tf.push_character(character)?.last()?.clone()))
          .unwrap_or(tf);
        let buf = tf
          .context()
          .result()
          .stopped_buffer()
          .unwrap_or(String::new());
        let context = last.context().push_result_string(buf);
        tf.set_context(context);

        Some(vec![tf])
      }
      _ => Some(self.replace_last_element(tfs)),
    }
  }
}

impl Displayable for HenkanTransformer {
  fn buffer_content(&self) -> String {
    self
      .stack
      .last()
      .and_then(|tf| Some(tf.buffer_content()))
      .unwrap_or(String::new())
  }

  fn display_string(&self) -> String {
    self
      .stack
      .last()
      .and_then(|tf| Some(tf.display_string()))
      .unwrap_or(String::new())
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
      ["Kanji\n", { stopped_buffer: "かんじ", display: "", transformer_type: Stopped(Compleated) }],
      ["Kanji \n", { stopped_buffer: "漢字", display: "", transformer_type: Stopped(Compleated) }],
      ["Kanji a", { stopped_buffer: "漢字あ", display: "", transformer_type: Stopped(Compleated) }],
      ["Kanji A", { stopped_buffer: "漢字", display: "▽あ", transformer_type: Henkan }],
      ["Kanji k", { stopped_buffer: "漢字", display: "k", transformer_type: Hiragana }],
      ["Kanji K", { stopped_buffer: "漢字", display: "▽k", transformer_type: Henkan }],
      ["Kanji Kanji", { stopped_buffer: "漢字", display: "▽かんじ", transformer_type: Henkan }],
      ["Michi \n", { stopped_buffer: "未知", display: "", transformer_type: Stopped(Compleated) }],
      ["Michigo ", { display: "[登録: みちご]", transformer_type: Henkan }],
      ["Michigo [escape]", { display: "▽みちご", transformer_type: Henkan }],
      ["Michigo [escape][backspace]", { display: "▽みち", transformer_type: Henkan }],
      ["Michigo [escape][backspace] ", { display: "▼未知", transformer_type: Henkan }],
      ["Michigo [escape][backspace] \n", { stopped_buffer: "未知", display: "", transformer_type: Stopped(Compleated) }],
      ["Michigo  [escape]", { display: "▽みちご", transformer_type: Henkan }],
      ["Michigo  [escape][escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["Michigo  [escape]a", { display: "▽みちごあ", transformer_type: Henkan }],
    ];
    crate::tests::helpers::TestData::batch(vec);

    // TODO: カタカナ時のテスト
  }
}

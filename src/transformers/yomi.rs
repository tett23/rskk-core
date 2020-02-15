use kana::{half2kana, hira2kata, kata2hira};

use super::tables::LetterType;
use super::{
  AsTransformerTrait, Displayable, KeyCode, SelectCandidateTransformer, Stackable, Transformable,
  TransformerTypes, UnknownWordTransformer, WithContext, Word,
};
use crate::Context;

#[derive(Clone, Debug)]
pub struct YomiTransformer {
  context: Context,
  current_transformer_type: TransformerTypes,
  word: Word,
}

impl YomiTransformer {
  pub fn new(context: Context, transformer_type: TransformerTypes) -> Self {
    YomiTransformer {
      context,
      current_transformer_type: transformer_type,
      word: Word::new(match transformer_type {
        TransformerTypes::Hiragana => LetterType::Hiragana,
        TransformerTypes::Katakana => LetterType::Katakana,
        TransformerTypes::EnKatakana => unimplemented!(),
        TransformerTypes::EmEisu => unimplemented!(),
        _ => unreachable!(),
      }),
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
      .dictionary()
      .transform(self.word.to_dic_read()?)
      .map(|dic_entry| {
        SelectCandidateTransformer::new(self.new_context(), dic_entry, self.word.clone())
      })
  }

  fn transition_to_unknown_word(&self) -> UnknownWordTransformer {
    UnknownWordTransformer::new(self.new_context(), { self.word.clone() })
  }
}

impl WithContext for YomiTransformer {
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

impl Transformable for YomiTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Yomi
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    let mut tf = self.clone();
    tf.word.push(character);

    if tf.word.is_stopped() {
      let mut tf2 = tf.clone();
      tf2.word.remove_okuri();

      Some(vec![box tf2.clone(), tf.try_composition()])
    } else {
      Some(vec![box tf])
    }
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(if self.word.has_okuri() {
      vec![self.pop().0]
    } else {
      vec![]
    })
  }

  fn push_space(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let mut tf = self.clone();
    tf.word.remove_okuri();

    Some(vec![box tf, self.try_composition()])
  }

  fn push_enter(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let mut tf = self.clone();
    tf.word.remove_okuri();

    Some(vec![
      self.to_completed_with_update_buffer(tf.word.buffer_content())
    ])
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    if self.word.is_empty() {
      return Some(vec![]);
    }

    let mut tf = self.clone();
    tf.word.pop();

    Some(vec![box tf])
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.push_backspace()
  }

  fn push_any_character(&self, key: &KeyCode) -> Option<Vec<Box<dyn Transformable>>> {
    match key.printable_key() {
      Some('q') => Some(vec![self.to_completed_with_update_buffer(
        match self.current_transformer_type {
          TransformerTypes::Hiragana => hira2kata(&self.buffer_content()),
          TransformerTypes::Katakana => kata2hira(&self.buffer_content()),
          TransformerTypes::EnKatakana => half2kana(&self.buffer_content()),
          _ => return None,
        },
      )]),
      _ => None,
    }
  }
}

impl Displayable for YomiTransformer {
  fn buffer_content(&self) -> String {
    self.word.buffer_content()
  }

  fn display_string(&self) -> String {
    "▽".to_owned() + &self.word.display_string()
  }
}

impl AsTransformerTrait for YomiTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
}

impl Stackable for YomiTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!();
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut tf = self.clone();
    tf.word.pop();
    if tf.word.is_empty() {
      return (self.to_canceled(), None);
    }

    (box tf, None)
  }

  fn replace_last_element(&self, _: Vec<Box<dyn Transformable>>) -> Vec<Box<dyn Transformable>> {
    unreachable!();
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    vec![]
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

    let vec = crate::tds![conf, YomiTransformer, Hiragana;
      ["[escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["hiragana", { display: "▽ひらがな", transformer_type: Yomi }],
      ["hiragana\n", { stopped_buffer: "ひらがな", transformer_type: Stopped(Compleated) }],
      ["oku[escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["okuR", { display: "▽おく*r", transformer_type: Yomi }],
      ["okuR[escape]", { display: "▽おく", transformer_type: Yomi }],
      ["okuR\n", { stopped_buffer: "おく", transformer_type: Stopped(Compleated) }],
      ["okuRi", { display: "▼送り", transformer_type: SelectCandidate }],
      ["kannji ", { display: "▼漢字", transformer_type: SelectCandidate }],
      ["kannji [escape]", { display: "", transformer_type: Stopped(Canceled) }],
      ["michigo ", { display: "[登録: みちご]", transformer_type: UnknownWord }],
      ["aA", { display: "[登録: あ*あ]", transformer_type: UnknownWord }],
      ["aKa", { display: "[登録: あ*か]", transformer_type: UnknownWord }],
      ["aTte", { display: "[登録: あ*って]", transformer_type: UnknownWord }],
      ["aTsu", { display: "[登録: あ*つ]", transformer_type: UnknownWord }],
      ["a[backspace]", { display: "▽", transformer_type: Yomi }],
      ["aa[backspace]", { display: "▽あ", transformer_type: Yomi }],
      ["aa[backspace]a", { display: "▽ああ", transformer_type: Yomi }],
      ["aa[backspace][backspace]i", { display: "▽い", transformer_type: Yomi }],
      ["a[backspace][backspace]", { display: "", transformer_type: Stopped(Canceled) }],
      ["aK", { display: "▽あ*k", transformer_type: Yomi }],
      ["aK[backspace]", { display: "▽あ", transformer_type: Yomi }],
      ["aK[backspace][backspace]", { display: "▽", transformer_type: Yomi }],
      ["aK[backspace][backspace]a", { display: "▽あ", transformer_type: Yomi }],
      ["aK[backspace][backspace]K", { display: "▽k", transformer_type: Yomi }],
      ["henka[backspace][backspace]", { display: "▽へ", transformer_type: Yomi }],
      ["katakanaq", { stopped_buffer: "カタカナ", transformer_type: Stopped(Compleated) }],
    ];
    crate::tests::helpers::TestData::batch(vec);

    // TODO: カタカナ時のテスト
  }
}

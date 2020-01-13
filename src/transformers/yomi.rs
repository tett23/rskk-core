use super::{
  AsTransformerTrait, Config, Displayable, SelectCandidateTransformer, Stackable,
  StoppedTransformer, Transformable, TransformerTypes, UnknownWordTransformer, WithConfig, Word,
};

#[derive(Clone, Debug)]
pub struct YomiTransformer {
  config: Config,
  current_transformer_type: TransformerTypes,
  word: Word,
}

impl YomiTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    YomiTransformer {
      config: config.clone(),
      current_transformer_type: transformer_type,
      word: Word::new(),
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
      .config
      .dictionary()
      .transform(self.word.to_dic_read()?)
      .map(|dic_entry| SelectCandidateTransformer::new(self.config(), dic_entry, self.word.clone()))
  }

  fn transition_to_unknown_word(&self) -> UnknownWordTransformer {
    UnknownWordTransformer::new(self.config(), { self.word.clone() })
  }
}

impl WithConfig for YomiTransformer {
  fn config(&self) -> Config {
    self.config.clone()
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

    Some(vec![box StoppedTransformer::completed(
      self.config(),
      tf.word.buffer_content(),
    )])
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
  use crate::tds;
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, YomiTransformer, Hiragana;
      ["[escape]", "", Stopped(Canceled)],
      ["hiragana", "▽ひらがな", Yomi],
      ["hiragana\n", "ひらがな", Stopped(Compleated)],
      ["oku[escape]", "", Stopped(Canceled)],
      ["okuR", "▽おく*r", Yomi],
      ["okuR[escape]", "▽おく", Yomi],
      ["okuR\n", "おく", Stopped(Compleated)],
      ["okuRi", "▼送り", SelectCandidate],
      ["kannji ", "▼漢字", SelectCandidate],
      ["kannji [escape]", "", Stopped(Canceled)],
      ["michigo ", "[登録: みちご]", UnknownWord],
      ["aA", "[登録: あ*あ]", UnknownWord],
      ["aKa", "[登録: あ*か]", UnknownWord],
      ["aTte", "[登録: あ*って]", UnknownWord],
      ["aTsu", "[登録: あ*つ]", UnknownWord],
      ["a[backspace]", "▽", Yomi],
      ["aa[backspace]", "▽あ", Yomi],
      ["aa[backspace]a", "▽ああ", Yomi],
      ["aa[backspace][backspace]i", "▽い", Yomi],
      ["a[backspace][backspace]", "", Stopped(Canceled)],
      ["aK", "▽あ*k", Yomi],
      ["aK[backspace]", "▽あ", Yomi],
      ["aK[backspace][backspace]", "▽", Yomi],
      ["aK[backspace][backspace]a", "▽あ", Yomi],
      ["aK[backspace][backspace]K", "▽k", Yomi],
      ["henka[backspace][backspace]", "▽へ", Yomi],
    ];
    test_transformer(items);

    // TODO: カタカナ時のテスト
  }
}

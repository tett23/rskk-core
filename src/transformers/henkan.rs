use super::{
  AsTransformerTrait, Config, Displayable, SelectCandidateTransformer, Stackable, StoppedReason,
  StoppedTransformer, Transformable, TransformerTypes, UnknownWordTransformer, WithConfig, Word,
  YomiTransformer,
};
use crate::keyboards::{KeyCode, Keyboard};
use StoppedReason::*;

#[derive(Clone)]
pub struct HenkanTransformer {
  config: Config,
  current_transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
}

impl HenkanTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    HenkanTransformer {
      config: config.clone(),
      current_transformer_type: transformer_type,
      stack: vec![Box::new(YomiTransformer::new(config, transformer_type))],
    }
  }
}

impl WithConfig for HenkanTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for HenkanTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Henkan
  }

  fn try_change_transformer(
    &self,
    keyboard: &Box<dyn Keyboard>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    let new_transformer = self
      .send_target()
      .try_change_transformer(keyboard, last_key_code);

    Some(self.replace_last_element(new_transformer?))
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let new_transformer = self.send_target().push_character(character);
    if new_transformer.transformer_type() != TransformerTypes::OkuriCompleted {
      return self.replace_last_element(new_transformer);
    }

    let (yomi, _) = YomiTransformer::from_pair(
      self.config(),
      self.current_transformer_type,
      new_transformer.pair(),
    )
    .pop();

    let buf = self.buffer_content();
    let tf: Box<dyn Transformable> = match self.config.dictionary.transform(&buf) {
      Some(dic_entry) => {
        box SelectCandidateTransformer::new(self.config(), dic_entry, Some(character))
      }
      None => box UnknownWordTransformer::new(self.config(), Word::from(new_transformer.pair())),
    };

    self.replace_last_element(yomi).push(tf)
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    let new_transformer = self.send_target().push_escape();
    match new_transformer.transformer_type() {
      TransformerTypes::Stopped(Canceled) => self.pop().0,
      _ => self.replace_last_element(new_transformer),
    }
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    let new_transformer = self.send_target().push_enter();
    match new_transformer.transformer_type() {
      TransformerTypes::Stopped(Compleated) => new_transformer,
      TransformerTypes::Stopped(Canceled) => self.pop().0,
      _ => self.replace_last_element(new_transformer),
    }
  }

  fn push_space(&self) -> Box<dyn Transformable> {
    self.replace_last_element(self.send_target().push_space())
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    self.replace_last_element(self.send_target().push_backspace())
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
      None => box StoppedTransformer::empty(self.config()),
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

  fn replace_last_element(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    if ret.stack.len() == 1 && item.transformer_type() == TransformerTypes::Stopped(Canceled) {
      return self.to_canceled();
    }

    ret.stack.pop();
    ret.stack.push(item);

    box ret
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    self.stack.clone()
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

    let items = tds![conf, HenkanTransformer, Hiragana;
      ["hiragana", "▽ひらがな", Henkan],
      ["hiragana\n", "ひらがな", Stopped(Compleated)],
      ["hiragana[escape]", "", Stopped(Canceled)],
      ["kannji ", "▼漢字", Henkan],
      ["kannji \n", "漢字", Stopped(Compleated)],
      ["okuR", "▽おく*r", Henkan],
      ["okuR\n", "▽おく", Henkan],
      ["okuR[escape]", "▽おく", Henkan],
      ["okuRi", "▼送り", Henkan],
      ["okuRi[escape]", "▽おく", Henkan],
      ["okuRi\n", "送り", Stopped(Compleated)],
      ["michigo ", "[登録: みちご]", Henkan],
      ["michigo ", "[登録: みちご]", Henkan],
      ["michigo [backspace]", "[登録: みちご]", Henkan],
      ["aa[backspace]", "▽あ", Henkan],
      ["aa[backspace]", "▽あ", Henkan],
      ["aa[backspace][backspace]", "▽", Henkan],
      ["aa[backspace][backspace][backspace]", "", Stopped(Canceled)],
    ];
    test_transformer(items);

    // TODO: カタカナ時のテスト
  }
}

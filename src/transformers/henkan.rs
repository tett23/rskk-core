use super::{
  AsTransformerTrait, Config, Displayable, KeyCode, Stackable, StoppedTransformer, Transformable,
  TransformerTypes, WithConfig, YomiTransformer,
};

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
      stack: vec![box YomiTransformer::new(config, transformer_type)],
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
      ["kannji [backspace]", "▽かんじ", Henkan],
      ["kannji \n", "漢字", Stopped(Compleated)],
      ["okuR", "▽おく*r", Henkan],
      ["okuR\n", "おく", Stopped(Compleated)],
      ["okuR[escape]", "▽おく", Henkan],
      ["okuRi", "▼送り", Henkan],
      ["okuRi[escape]", "▽おく", Henkan],
      ["okuRi\n", "送り", Stopped(Compleated)],
      ["okuRia", "送り", Stopped(Compleated)],
      ["michigo ", "[登録: みちご]", Henkan],
      ["aA", "[登録: あ*あ]", Henkan],
      ["michigo [backspace]", "[登録: みちご]", Henkan],
      ["aa[backspace]", "▽あ", Henkan],
      ["aa[backspace][backspace]", "▽", Henkan],
      ["aa[backspace][backspace][backspace]", "", Stopped(Canceled)],
      ["aA", "[登録: あ*あ]", Henkan],
      ["aA[escape]", "▽あ", Henkan],
      ["aKa", "[登録: あ*か]", Henkan],
      ["aA[escape]", "▽あ", Henkan],
      ["aTte", "[登録: あ*って]", Henkan],
      ["aA[escape]", "▽あ", Henkan],
      ["aTsu", "[登録: あ*つ]", Henkan],
    ];
    test_transformer(items);

    // TODO: カタカナ時のテスト
  }
}

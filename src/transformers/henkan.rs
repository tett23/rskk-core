use super::{
  AsTransformerTrait, CanceledTransformer, Config, Displayable, MetaKey,
  SelectCandidateTransformer, Stackable, StoppedTransformer, Transformable, TransformerTypes,
  UnknownWordTransformer, WithConfig, Word, YomiTransformer,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct HenkanTransformer {
  config: Config,
  transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
}

impl HenkanTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    HenkanTransformer {
      config: config.clone(),
      transformer_type: transformer_type.clone(),
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
    pressing_keys: &HashSet<KeyCode>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    let new_transformer = self
      .send_target()
      .try_change_transformer(pressing_keys, last_key_code);

    match new_transformer {
      Some(tf) => Some(self.replace_last_element(tf)),
      None => None,
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let new_transformer = self.send_target().push_character(character);
    if new_transformer.transformer_type() == TransformerTypes::OkuriCompleted {
      let buf = self.buffer_content();
      let tf: Box<dyn Transformable> = match self.config.dictionary.transform(&buf) {
        Some(dic_entry) => Box::new(SelectCandidateTransformer::new(
          self.config(),
          dic_entry,
          Some(character),
        )),
        None => Box::new(UnknownWordTransformer::new(
          self.config(),
          Word::new(&buf, None),
        )),
      };

      return self.push(tf);
    }

    self.replace_last_element(new_transformer)
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformable> {
    let target = self.send_target();

    let new_transformer = match key_code {
      KeyCode::PrintableMeta(MetaKey::Space, _) | KeyCode::Meta(MetaKey::Space) => {
        self.push_space()
      }
      _ => target.push_meta_key(key_code),
    };

    self.transformer_updated(new_transformer)
  }

  fn push_space(&self) -> Box<dyn Transformable> {
    let buf = self.buffer_content();
    match self.config.dictionary.transform(&buf) {
      Some(dic_entry) => Box::new(SelectCandidateTransformer::new(
        self.config(),
        dic_entry,
        None,
      )),
      None => Box::new(UnknownWordTransformer::new(
        self.config(),
        Word::new(&buf, None),
      )),
    }
  }

  fn transformer_updated(&self, new_transformer: Box<dyn Transformable>) -> Box<dyn Transformable> {
    match (
      new_transformer.is_stopped(),
      new_transformer.transformer_type(),
    ) {
      (true, TransformerTypes::Stopped) => new_transformer,
      (true, TransformerTypes::Canceled) => self.pop().0,
      _ => self.replace_last_element(new_transformer),
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
    Box::new(self.clone())
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.clone(),
      None => Box::new(StoppedTransformer::empty(self.config())),
    }
  }
}

impl Stackable for HenkanTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.stack.push(item);

    Box::new(ret)
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();

    let item = ret.stack.pop();
    if ret.stack.len() == 0 {
      return (Box::new(CanceledTransformer::new(self.config())), None);
    }

    (Box::new(ret), item)
  }

  fn replace_last_element(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.stack.pop();
    ret.stack.push(item);

    Box::new(ret)
  }
}

#[cfg(test)]
mod tests {
  use crate::tds;
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, HenkanTransformer, Hiragana;
      ["hiragana", "▽ひらがな", Henkan],
      ["hiragana\n", "ひらがな", Stopped],
      ["hiragana[escape]", "", Canceled],
      ["kannji ", "▼漢字", Henkan],
      ["kannji \n", "漢字", Stopped],
      ["okuR", "▽おく*r", Henkan],
      ["okuR\n", "▽おく", Henkan],
      ["okuR[escape]", "▽おく", Henkan],
      ["okuRi", "▼送り", Henkan],
      ["okuRi[escape]", "▽おく*r", Henkan],
      ["okuRi\n", "送り", Stopped],
      ["michigo ", "[登録: みちご]", Henkan],
    ];
    test_transformer(items);

    // TODO: カタカナ時のテスト
  }
}

use super::{
  AsTransformerTrait, CanceledTransformer, Config, Displayable, MetaKey, Stackable,
  StoppedTransformer, Transformable, TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use crate::tf;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct ContinuousTransformer {
  config: Config,
  current_transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
}

impl ContinuousTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    ContinuousTransformer {
      config: config.clone(),
      current_transformer_type: transformer_type,
      stack: vec![tf!(config, transformer_type)],
    }
  }

  fn stopped_buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .filter(|tf| tf.is_stopped())
      .fold("".to_string(), |acc, tf| acc + &tf.buffer_content())
  }

  fn is_empty(&self) -> bool {
    self.buffer_content() == ""
  }
}

impl WithConfig for ContinuousTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for ContinuousTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Continuous
  }

  fn try_change_transformer(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    match self.is_empty() {
      true => {
        let new_transformer = self
          .send_target()
          .try_change_transformer(pressing_keys, last_key_code);

        match new_transformer {
          Some(tf) if tf.transformer_type() == TransformerTypes::Henkan => Some(tf),
          Some(tf) => Some(self.replace_last_element(tf)),
          None => None,
        }
      }
      false => None,
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let new_transformer = self.send_target().push_character(character);
    match new_transformer.is_stopped() {
      true => {
        let mut ret = self.clone();
        ret.stack.pop();
        ret.stack.push(new_transformer);
        ret
          .stack
          .push(tf!(self.config(), self.current_transformer_type));

        Box::new(ret)
      }
      false => self.replace_last_element(new_transformer),
    }
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformable> {
    let target = self.send_target();

    let new_transformer = match key_code {
      KeyCode::PrintableMeta(MetaKey::Enter, _) | KeyCode::Meta(MetaKey::Enter) => {
        return self.push_enter()
      }
      _ => target.push_meta_key(key_code),
    };

    self.transformer_updated(new_transformer)
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.push_escape(),
      None => Box::new(CanceledTransformer::new(self.config().clone())),
    }
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    Box::new(StoppedTransformer::new(
      self.config(),
      self.stopped_buffer_content(),
    ))
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    // TODO: stackが空になるまでstack先頭にbackspaceを送り続ける
    // すべて空のときは空のStoppedを返す
    unimplemented!()
  }

  fn push_delete(&self) -> Box<dyn Transformable> {
    self.push_backspace()
  }

  fn transformer_updated(&self, new_transformer: Box<dyn Transformable>) -> Box<dyn Transformable> {
    match new_transformer.is_stopped() {
      true if new_transformer.transformer_type() == TransformerTypes::Canceled => {
        Box::new(CanceledTransformer::new(self.config()))
      }
      true => Box::new(StoppedTransformer::new(
        self.config(),
        self.replace_last_element(new_transformer).buffer_content(),
      )),
      false => self.replace_last_element(new_transformer),
    }
  }
}

impl Displayable for ContinuousTransformer {
  fn buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .fold("".to_string(), |acc, tf| acc + &tf.buffer_content())
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }
}

impl AsTransformerTrait for ContinuousTransformer {
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

impl Stackable for ContinuousTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.stack.push(item);

    Box::new(ret)
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();

    let item = ret.stack.pop();

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
  use super::*;
  use crate::tds;
  use crate::tests::{dummy_conf, test_transformer};
  use TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, ContinuousTransformer, Hiragana;
        ["hiragana", "ひらがな", Continuous],
        ["hiragana\n", "ひらがな", Stopped],
        ["Kannji", "▽かんじ", Henkan],
        ["Kannji \n", "漢字", Stopped],
    ];
    test_transformer(items);
  }
}

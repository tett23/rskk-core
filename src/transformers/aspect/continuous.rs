use super::super::{
  AsTransformerTrait, Canceled, Config, Displayable, Stackable, Stopped, Transformable,
  TransformerState, TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct ContinuousTransformer {
  config: Config,
  transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
}

impl ContinuousTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    ContinuousTransformer {
      config: config.clone(),
      transformer_type,
      stack: vec![transformer_type.to_transformer(config)],
    }
  }

  fn new_child_transformer(&self) -> Box<dyn Transformable> {
    self.transformer_type.to_transformer(self.config.clone())
  }

  fn new_from_stack(&self, stack: Vec<Box<dyn Transformable>>) -> Self {
    let mut ret = self.clone();
    ret.stack = stack;

    ret
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

impl TransformerState for ContinuousTransformer {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformable for ContinuousTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::ContinuousTransformer
  }

  fn try_change_transformer(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    match self.is_empty() {
      true => self
        .send_target()
        .try_change_transformer(pressing_keys, last_key_code),
      false => None,
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let last_tf = self.stack.last();
    if last_tf.is_none() {
      return Box::new(Stopped::empty(self.config.clone()));
    }
    let last_tf = last_tf.unwrap();

    let new_transformer = last_tf.push_character(character);
    match new_transformer.is_stopped() {
      true => {
        let mut ret = self.clone();
        ret.stack.pop();
        ret.stack.push(new_transformer);
        ret.stack.push(ret.new_child_transformer());

        Box::new(ret)
      }
      false => self.replace_last_element(new_transformer),
    }
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.push_escape(),
      None => Box::new(Canceled::new(self.config().clone())),
    }
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    Box::new(Stopped::new(self.config(), self.stopped_buffer_content()))
  }

  fn push_space(&self) -> Box<dyn Transformable> {
    unimplemented!()
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    // TODO: stackが空になるまでstack先頭にbackspaceを送り続ける
    // すべて空のときは空のStoppedを返す
    unimplemented!()
  }

  fn push_delete(&self) -> Box<dyn Transformable> {
    self.push_backspace()
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
    self
      .stack
      .iter()
      .fold("".to_string(), |acc, tf| acc + &tf.display_string())
  }
}

impl AsTransformerTrait for ContinuousTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

impl Stackable for ContinuousTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.new_from_stack(self.stack.clone());

    ret.stack.push(item);

    Box::new(ret)
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.new_from_stack(self.stack.clone());

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
  use crate::tests::dummy_conf;

  #[test]
  fn push_character() {
    let config = dummy_conf();
    let continuous = ContinuousTransformer::new(config.clone(), TransformerTypes::Hiragana);

    let continuous = continuous.push_character('h');
    let continuous = continuous.push_character('i');
    let continuous = continuous.push_character('r');
    let continuous = continuous.push_character('a');
    let continuous = continuous.push_character('g');
    let continuous = continuous.push_character('a');
    let continuous = continuous.push_character('n');
    let continuous = continuous.push_character('a');

    assert_eq!(continuous.display_string(), "ひらがな");
    assert_eq!(
      continuous.transformer_type(),
      TransformerTypes::ContinuousTransformer
    );

    let continuous = continuous.push_enter();

    assert_eq!(continuous.display_string(), "ひらがな");
    assert_eq!(continuous.transformer_type(), TransformerTypes::Stopped);
  }
}

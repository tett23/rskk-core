use super::{
  AsTransformerTrait, Config, Displayable, Stackable, Transformable, TransformerTypes, WithConfig,
};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum StoppedReason {
  Compleated,
  Canceled,
}

#[derive(Clone)]
pub struct StoppedTransformer {
  config: Config,
  reason: StoppedReason,
  buffer: String,
}

impl StoppedTransformer {
  pub fn new<S: Into<String>>(config: Config, reason: StoppedReason, buffer: S) -> Self {
    StoppedTransformer {
      config,
      reason,
      buffer: buffer.into(),
    }
  }

  pub fn completed<S: Into<String>>(config: Config, buffer: S) -> Self {
    Self::new(config, StoppedReason::Compleated, buffer)
  }

  pub fn empty(config: Config) -> Self {
    Self::new(config, StoppedReason::Compleated, "")
  }

  pub fn canceled(config: Config) -> Self {
    Self::new(config, StoppedReason::Canceled, "")
  }
}

impl WithConfig for StoppedTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for StoppedTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Stopped(self.reason)
  }

  fn push_character(&self, _: char) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let tf = self.pop().0;
    if tf.is_canceled() {
      return Some(vec![]);
    }

    Some(vec![tf])
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.push_backspace()
  }
}

impl Displayable for StoppedTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

impl AsTransformerTrait for StoppedTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }
}

impl Stackable for StoppedTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!()
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();
    ret.buffer.pop();

    if ret.buffer.len() == 0 {
      return (
        box StoppedTransformer::canceled(self.config()),
        Some(box StoppedTransformer::canceled(self.config())),
      );
    }

    (
      box ret,
      Some(box StoppedTransformer::canceled(self.config())),
    )
  }

  fn replace_last_element(&self, _: Vec<Box<dyn Transformable>>) -> Vec<Box<dyn Transformable>> {
    vec![box self.clone()]
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    vec![]
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::dummy_conf;
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn stack() {
    let conf = dummy_conf();

    let tf = StoppedTransformer::completed(conf.clone(), "aa").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Compleated));
    assert_eq!(tf.buffer_content(), "a");

    let tf = StoppedTransformer::completed(conf.clone(), "a").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = StoppedTransformer::canceled(conf.clone()).pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");
  }
}

use std::rc::Rc;

use super::{
  AsTransformerTrait, Displayable, Stackable, Transformable, TransformerTypes, WithContext,
};
use crate::Context;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum StoppedReason {
  Compleated,
  Canceled,
}

#[derive(Clone)]
pub struct StoppedTransformer {
  context: Rc<Context>,
  reason: StoppedReason,
  buffer: String,
}

impl StoppedTransformer {
  pub fn new<S: Into<String>>(context: Rc<Context>, reason: StoppedReason, buffer: S) -> Self {
    StoppedTransformer {
      context,
      reason,
      buffer: buffer.into(),
    }
  }

  pub fn completed<S: Into<String>>(context: Rc<Context>, buffer: S) -> Self {
    Self::new(context, StoppedReason::Compleated, buffer)
  }

  pub fn empty(context: Rc<Context>) -> Self {
    Self::new(context, StoppedReason::Compleated, "")
  }

  pub fn canceled(context: Rc<Context>) -> Self {
    Self::new(context, StoppedReason::Canceled, "")
  }
}

impl WithContext for StoppedTransformer {
  fn context(&self) -> &Context {
    &self.context
  }

  fn clone_context(&self) -> Rc<Context> {
    self.context.clone()
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
        box StoppedTransformer::canceled(self.clone_context()),
        Some(box StoppedTransformer::canceled(self.clone_context())),
      );
    }

    (
      box ret,
      Some(box StoppedTransformer::canceled(self.clone_context())),
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
  use crate::tests::dummy_context;
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn stack() {
    let conf = dummy_context();

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

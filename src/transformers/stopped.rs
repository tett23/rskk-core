use std::cell::RefCell;
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
  context: Rc<RefCell<Context>>,
  reason: StoppedReason,
}

impl StoppedTransformer {
  pub fn new(context: Rc<RefCell<Context>>, reason: StoppedReason) -> Self {
    StoppedTransformer { context, reason }
  }

  pub fn completed(context: Rc<RefCell<Context>>) -> Self {
    Self::new(context, StoppedReason::Compleated)
  }

  pub fn canceled(context: Rc<RefCell<Context>>) -> Self {
    Self::new(context, StoppedReason::Canceled)
  }
}

impl WithContext for StoppedTransformer {
  fn clone_context(&self) -> Rc<RefCell<Context>> {
    self.context.clone()
  }

  #[cfg(test)]
  fn set_context(&mut self, context: Rc<RefCell<Context>>) {
    self.context = context;
  }
}

impl Transformable for StoppedTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Stopped(self.reason)
  }

  fn push_character(&self, _: char) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
}

impl Displayable for StoppedTransformer {
  fn buffer_content(&self) -> String {
    String::new()
  }

  fn display_string(&self) -> String {
    String::new()
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
    unreachable!()
  }

  fn replace_last_element(&self, _: Vec<Box<dyn Transformable>>) -> Vec<Box<dyn Transformable>> {
    vec![box self.clone()]
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    vec![]
  }
}

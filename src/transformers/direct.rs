use std::cell::RefCell;
use std::rc::Rc;

use super::tables::{BufferPairs, LetterType};
use super::{
  AsTransformerTrait, Displayable, Stackable, Transformable, TransformerTypes, WithContext,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::{set, tf, Context};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct DirectTransformer {
  context: Rc<RefCell<Context>>,
  buffer: BufferPairs,
}

impl DirectTransformer {
  pub fn new(context: Rc<RefCell<Context>>) -> Self {
    DirectTransformer {
      context,
      buffer: BufferPairs::new(LetterType::Direct),
    }
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![TransformerTypes::Hiragana]
  }
}

impl WithContext for DirectTransformer {
  fn clone_context(&self) -> Rc<RefCell<Context>> {
    self.context.clone()
  }

  #[cfg(test)]
  fn set_context(&mut self, context: Rc<RefCell<Context>>) {
    self.context = context;
  }
}

impl Transformable for DirectTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Direct
  }

  fn try_change_transformer(
    &self,
    keyboard: &Box<dyn Keyboard>,
    _: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    let transformer_type = self
      .context
      .borrow()
      .config()
      .key_config()
      .try_change_transformer(&Self::allow_transformers(), keyboard.pressing_keys());

    Some(tf!(self.clone_context(), transformer_type?))
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    let mut tf = self.clone();

    tf.buffer.push(character);
    dbg!(&tf.buffer);
    Some(vec![match tf.buffer.is_stopped() {
      true => tf.to_completed_with_update_buffer(tf.buffer.to_string()),
      false => box tf,
    }])
  }
}

impl Stackable for DirectTransformer {
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

  fn child_transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Direct
  }
}

impl Displayable for DirectTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.to_string()
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }
}

impl AsTransformerTrait for DirectTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
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

    let vec = crate::tds!(conf, Direct;
      ["[escape]", { display: "", transformer_type: Direct }],
      ["a", { display: "", stopped_buffer: "a", transformer_type: Stopped(Compleated) }],
      ["A", { display: "", stopped_buffer: "A", transformer_type: Stopped(Compleated) }],
      ["!", { display: "", stopped_buffer: "!", transformer_type: Stopped(Compleated) }],
    );
    crate::tests::helpers::TestData::batch(vec);
  }
}

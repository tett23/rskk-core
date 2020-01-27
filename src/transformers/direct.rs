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
  context: Rc<Context>,
  buffer: BufferPairs,
}

impl DirectTransformer {
  pub fn new(context: Rc<Context>) -> Self {
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
  fn context(&self) -> &Context {
    &self.context
  }

  fn clone_context(&self) -> Rc<Context> {
    self.context.clone()
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
      .config()
      .key_config()
      .try_change_transformer(&Self::allow_transformers(), keyboard.pressing_keys());

    Some(tf!(self.clone_context(), transformer_type?))
  }

  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>> {
    let mut tf = self.clone();

    tf.buffer.push(character);
    Some(vec![match tf.buffer.is_stopped() {
      true => tf.to_completed(),
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
  use crate::tds;
  use crate::tests::{dummy_context, test_transformer};
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_context();

    let items = tds![conf, Direct;
      ["[escape]", "", Direct],
      ["a", "a", Stopped(Compleated)],
      ["A", "A", Stopped(Compleated)],
      ["!", "!", Stopped(Compleated)],
    ];
    test_transformer(items);
  }
}

use super::{
  AsTransformerTrait, Config, Displayable, Stackable, StoppedTransformer, Transformable,
  TransformerTypes, WithConfig,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::{set, tf};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct DirectTransformer {
  config: Config,
  buffer: String,
}

impl DirectTransformer {
  pub fn new(config: Config) -> Self {
    DirectTransformer {
      config,
      buffer: "".to_string(),
    }
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![TransformerTypes::Hiragana]
  }
}

impl WithConfig for DirectTransformer {
  fn config(&self) -> Config {
    self.config.clone()
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
      .config
      .key_config()
      .try_change_transformer(&Self::allow_transformers(), keyboard.pressing_keys());

    Some(tf!(self.config(), transformer_type?))
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    return box StoppedTransformer::completed(self.config(), character.to_string());
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    self.to_canceled()
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    self.pop().0
  }
}

impl Stackable for DirectTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!()
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    (
      box StoppedTransformer::canceled(self.config()),
      Some(box StoppedTransformer::canceled(self.config())),
    )
  }

  fn replace_last_element(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    self.as_trait()
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
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

impl AsTransformerTrait for DirectTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;
  use crate::{tds, tfe};

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, Direct;
      ["[escape]", "", Stopped(Canceled)],
      ["a", "a", Stopped(Compleated)],
      ["A", "A", Stopped(Compleated)],
      ["!", "!", Stopped(Compleated)],
    ];
    test_transformer(items);
  }

  #[test]
  fn stack() {
    let conf = dummy_conf();

    let tf = tfe!(conf, Direct; "").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = tfe!(conf, Direct; "a").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");
  }
}

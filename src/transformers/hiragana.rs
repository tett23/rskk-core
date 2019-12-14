use super::tables::hiragana_convert;
use super::{
  AsTransformerTrait, BufferState, Config, Displayable, Stackable, StoppedTransformer,
  Transformable, TransformerTypes, WithConfig,
};
use crate::keyboards::{KeyCode, Keyboard};
use crate::{set, tf};
use std::collections::HashSet;
use BufferState::*;

#[derive(Clone)]
pub struct HiraganaTransformer {
  config: Config,
  buffer: String,
}

impl HiraganaTransformer {
  pub fn new(config: Config) -> Self {
    HiraganaTransformer {
      config,
      buffer: "".to_string(),
    }
  }

  pub fn from_buffer<S: Into<String>>(config: Config, buffer: S) -> Self {
    let mut ret = Self::new(config);
    ret.buffer = buffer.into();

    ret
  }

  fn allow_transformers() -> HashSet<TransformerTypes> {
    set![
      TransformerTypes::Direct,
      TransformerTypes::Henkan,
      TransformerTypes::Abbr,
      TransformerTypes::Katakana,
      TransformerTypes::EnKatakana,
      TransformerTypes::EmEisu
    ]
  }
}

impl WithConfig for HiraganaTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for HiraganaTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Hiragana
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
    match transformer_type? {
      TransformerTypes::Henkan => {
        let tf = tf!(self.config(), transformer_type?);

        Some(tf.push_character(keyboard.last_character()?))
      }
      _ => Some(tf!(self.config(), transformer_type?)),
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    match hiragana_convert(&self.buffer, character) {
      Some((new_buffer, Continue)) => Box::new(Self::from_buffer(self.config(), new_buffer)),
      Some((new_buffer, Stop)) => {
        Box::new(StoppedTransformer::completed(self.config(), new_buffer))
      }
      None => Box::new(StoppedTransformer::canceled(self.config())),
    }
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    Box::new(StoppedTransformer::canceled(self.config()))
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    let mut buf = self.buffer_content();
    buf.pop();

    match buf.len() == 0 {
      true => Box::new(StoppedTransformer::canceled(self.config())),
      false => Box::new(Self::from_buffer(self.config(), buf)),
    }
  }

  fn push_delete(&self) -> Box<dyn Transformable> {
    self.push_backspace()
  }
}

impl Displayable for HiraganaTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

impl AsTransformerTrait for HiraganaTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

impl Stackable for HiraganaTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!()
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    if self.buffer.len() <= 1 {
      return (
        box StoppedTransformer::canceled(self.config()),
        Some(box StoppedTransformer::canceled(self.config())),
      );
    }

    let mut ret = self.clone();
    ret.buffer.pop();

    (
      Box::new(ret),
      Some(box StoppedTransformer::canceled(self.config())),
    )
  }

  fn replace_last_element(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    vec![]
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

    let items = tds![conf, Hiragana;
      ["a", "あ", Stopped(Compleated)],
      ["k", "k", Hiragana],
      ["k[escape]", "", Stopped(Canceled)],
      ["k[backspace]", "", Stopped(Canceled)],
      ["ts[backspace]", "t", Hiragana],
      ["ka", "か", Stopped(Compleated)],
      ["[backspace]", "", Stopped(Canceled)],
      ["k[escape]", "", Stopped(Canceled)],
      ["[escape]", "", Stopped(Canceled)],
      ["Kannji", "▽かんじ", Henkan],
    ];
    test_transformer(items);
  }

  #[test]
  fn stack() {
    let conf = dummy_conf();

    let tf = tfe!(conf, Hiragana; "").pop().0;
    assert_eq!(tf.transformer_type(), Stopped(Canceled));
    assert_eq!(tf.buffer_content(), "");

    let tf = tfe!(conf, Hiragana; "ts").pop().0;
    assert_eq!(tf.transformer_type(), Hiragana);
    assert_eq!(tf.buffer_content(), "t");
  }
}

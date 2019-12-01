use super::super::{
  AsTransformerTrait, Canceled, Config, Displayable, Stopped, Transformer, TransformerState,
  TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct ContinuousTransformer {
  config: Config,
  buffer: String,
  transformer: Box<dyn Transformer>,
  transformer_type: TransformerTypes,
}

impl ContinuousTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    ContinuousTransformer {
      config: config.clone(),
      buffer: "".to_string(),
      transformer: transformer_type.to_transformer(config),
      transformer_type,
    }
  }

  fn new_from_transformer(&self, transformer: Box<dyn Transformer>) -> Self {
    let mut ret = self.clone();
    ret.transformer = transformer;

    ret
  }

  fn new_from_buffer<S: Into<String>>(&self, buffer: S) -> Self {
    let mut ret = self.clone();
    ret.buffer = buffer.into();
    ret.transformer = self.transformer_type.to_transformer(self.config());

    ret
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

impl Transformer for ContinuousTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::ContinuousTransformer
  }

  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self.transformer.try_change_transformer(pressing_keys)
  }

  fn transformer_changed(
    &self,
    new_transformer: Box<dyn Transformer>,
    key: Option<char>,
  ) -> Box<dyn Transformer> {
    let new_transformer = match new_transformer.transformer_type() {
      TransformerTypes::Henkan => match key {
        Some(character) => new_transformer.push_character(character),
        None => new_transformer,
      },
      _ => new_transformer,
    };

    Box::new(self.new_from_transformer(new_transformer))
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    let new_transformer = self.transformer.push_character(character);

    match new_transformer.is_stopped() {
      true => {
        Box::new(self.new_from_buffer(self.buffer.clone() + &new_transformer.buffer_content()))
      }
      false => Box::new(self.new_from_transformer(new_transformer)),
    }
  }

  fn transformer_updated(&self, new_transformer: Box<dyn Transformer>) -> Box<dyn Transformer> {
    if new_transformer.is_stopped() {
      return new_transformer;
    }

    Box::new(self.new_from_transformer(new_transformer))
  }

  fn push_escape(&self) -> Box<dyn Transformer> {
    Box::new(Canceled::new(self.config()))
  }

  fn push_enter(&self) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.config(), self.buffer_content()))
  }

  fn push_space(&self) -> Box<dyn Transformer> {
    println!("push_space");
    self.transformer.push_space()
  }

  fn push_backspace(&self) -> Box<dyn Transformer> {
    match self.transformer.buffer_content().len() {
      0 => {
        let buf = self.buffer.clone();
        let (buf, _) = buf.split_at(buf.len() - 2);

        Box::new(self.new_from_buffer(buf))
      }
      _ => self.transformer.push_backspace(),
    }
  }

  fn push_delete(&self) -> Box<dyn Transformer> {
    self.push_backspace()
  }

  fn push_tab(&self) -> Box<dyn Transformer> {
    self.transformer.push_tab()
  }

  fn push_null(&self) -> Box<dyn Transformer> {
    self.transformer.push_null()
  }

  fn push_arrow_right(&self) -> Box<dyn Transformer> {
    unimplemented!()
  }

  fn push_arrow_down(&self) -> Box<dyn Transformer> {
    self.transformer.push_arrow_down()
  }

  fn push_arrow_left(&self) -> Box<dyn Transformer> {
    unimplemented!()
  }

  fn push_arrow_up(&self) -> Box<dyn Transformer> {
    self.transformer.push_arrow_up()
  }
}

impl Displayable for ContinuousTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.clone() + &self.transformer.buffer_content()
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }
}

impl AsTransformerTrait for ContinuousTransformer {
  fn as_trait(&self) -> Box<dyn Transformer> {
    Box::new(self.clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::dummy_conf;

  #[test]
  fn push() {
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

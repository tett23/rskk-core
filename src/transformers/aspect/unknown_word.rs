use super::super::{
  AsTransformerTrait, Canceled, Config, ContinuousTransformer, Displayable, Stopped, Transformer,
  TransformerState, TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Word(String, Option<String>);

impl Word {
  pub fn new<S: Into<String>>(yomi: S, okuri: Option<S>) -> Self {
    Word(
      yomi.into(),
      match okuri {
        Some(s) => Some(s.into()),
        None => None,
      },
    )
  }

  pub fn display_string(&self) -> String {
    match &self.1 {
      Some(okuri) => self.0.clone() + "*" + &okuri,
      None => self.0.clone(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct UnknownWord {
  config: Config,
  word: Word,
  buffer: String,
  transformer: Box<dyn Transformer>,
}

impl UnknownWord {
  pub fn new(config: Config, word: Word) -> Self {
    UnknownWord {
      config: config.clone(),
      word,
      buffer: "".to_string(),
      transformer: Box::new(ContinuousTransformer::new(
        config,
        TransformerTypes::Hiragana,
      )),
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
    ret.transformer = Box::new(ContinuousTransformer::new(
      self.config(),
      TransformerTypes::Hiragana,
    ));

    ret
  }
}

impl WithConfig for UnknownWord {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for UnknownWord {
  fn is_stopped(&self) -> bool {
    self.transformer.is_stopped()
  }
}

impl Transformer for UnknownWord {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::UnknownWord
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
    Box::new(self.new_from_transformer(self.transformer.push_character(character)))
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformer> {
    let new_transformer = self.transformer.push_meta_key(key_code);

    self.transformer_updated(new_transformer)
  }

  fn transformer_updated(&self, new_transformer: Box<dyn Transformer>) -> Box<dyn Transformer> {
    if new_transformer.is_stopped() {
      return new_transformer;
    }

    Box::new(self.new_from_transformer(new_transformer))
  }

  fn push_escape(&self) -> Box<dyn Transformer> {
    return Box::new(Canceled::new(self.config()));
  }

  fn push_enter(&self) -> Box<dyn Transformer> {
    if self.transformer.buffer_content().len() == 0 {
      return Box::new(Canceled::new(self.config()));
    }
    if self.transformer.buffer_content().len() == 0 {
      return Box::new(Stopped::new(self.config(), self.buffer_content()));
    }

    let new_transformer = self.transformer.push_enter();
    match new_transformer.is_stopped() {
      true => {
        Box::new(self.new_from_buffer(self.buffer_content() + &new_transformer.buffer_content()))
      }
      false => Box::new(self.new_from_transformer(new_transformer)),
    }
  }
}

impl Displayable for UnknownWord {
  fn buffer_content(&self) -> String {
    self.transformer.buffer_content()
  }

  fn display_string(&self) -> String {
    "[登録: ".to_string() + &self.word.display_string() + "]" + &self.transformer.display_string()
  }
}

impl AsTransformerTrait for UnknownWord {
  fn as_trait(&self) -> Box<dyn Transformer> {
    Box::new(self.clone())
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::keyboards::{KeyEvents, MetaKey};
//   use crate::set;
//   use crate::tests::dummy_conf;

//   #[test]
//   fn push() {
//     let config = dummy_conf();
//     let unknown_word = UnknownWord::new(config.clone(), Word::new("みちご", None));

//     let unknown_word = unknown_word.push_key_event(
//       &set![KeyCode::Meta(MetaKey::Shift)],
//       &KeyEvents::KeyDown(KeyCode::Printable('m')),
//       Some('m'),
//     );
//     let unknown_word = unknown_word.push_character('i');
//     let unknown_word = unknown_word.push_character('c');
//     let unknown_word = unknown_word.push_character('h');
//     let unknown_word = unknown_word.push_character('i');

//     let unknown_word = unknown_word.push_key_event(
//       &set![KeyCode::Meta(MetaKey::Shift)],
//       &KeyEvents::KeyDown(KeyCode::Printable('g')),
//       Some('g'),
//     );
//     let unknown_word = unknown_word.push_character('o');

//     assert_eq!(unknown_word.display_string(), "[登録: みちご]みちご");
//     assert_eq!(
//       unknown_word.transformer_type(),
//       TransformerTypes::UnknownWord,
//     );

//     let unknown_word = unknown_word.push_enter();

//     assert_eq!(unknown_word.display_string(), "未知語");
//     assert_eq!(unknown_word.transformer_type(), TransformerTypes::Stopped);
//   }
// }

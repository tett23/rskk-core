use super::super::{
  AsTransformerTrait, AspectTransformer, Config, Displayable, Transformer, TransformerState,
  TransformerTypes, WithConfig,
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
  transformer: Box<dyn Transformer>,
}

impl UnknownWord {
  pub fn new(config: Config, word: Word) -> Self {
    UnknownWord {
      config: config.clone(),
      word,
      transformer: Box::new(AspectTransformer::new(config, TransformerTypes::Hiragana)),
    }
  }

  fn new_from_transformer(&self, transformer: Box<dyn Transformer>) -> Self {
    let mut ret = self.clone();
    ret.transformer = transformer;

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

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    Box::new(self.new_from_transformer(self.transformer.push_character(character)))
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformer> {
    Box::new(self.new_from_transformer(self.transformer.push_meta_key(key_code)))
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::dummy_conf;

  #[test]
  fn push() {
    let config = dummy_conf();
    let unknown_word = UnknownWord::new(config.clone(), Word::new("みちご", None));

    let unknown_word = unknown_word.push_character('m');
    let unknown_word = unknown_word.push_character('i');
    let unknown_word = unknown_word.push_character('c');
    let unknown_word = unknown_word.push_character('h');
    let unknown_word = unknown_word.push_character('i');
    let unknown_word = unknown_word.push_character('g');
    let unknown_word = unknown_word.push_character('o');

    assert_eq!(unknown_word.display_string(), "[登録: みちご]▽みちご")
  }
}

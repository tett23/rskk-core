mod aspect;
mod direct;
mod henkan;
mod hiragana;
mod tables;

use crate::config::KeyConfig;
use crate::keyboards::{KeyCode, KeyCombinations};
use crate::{set, Config, Dictionary};
use objekt;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

pub use aspect::{Aspect, AspectTransformer, Canceled, SelectCandidate, Stopped, Yomi};
pub use direct::DirectTransformer;
pub use henkan::HenkanTransformer;
pub use hiragana::HiraganaTransformer;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum BufferState {
  Continue,
  Stop,
}

pub trait TransformerState {
  fn is_stopped(&self) -> bool;
}

pub trait Transformer: TransformerState + objekt::Clone {
  fn transformer_type(&self) -> TransformerTypes {
    unimplemented!()
  }
  fn push_character(&self, character: char) -> Box<dyn Transformer>;
  fn push_key_code(
    &self,
    pressing_keys: HashSet<KeyCode>,
    key_code: &KeyCode,
  ) -> Box<dyn Transformer>;
  fn buffer_content(&self) -> String;
  fn display_string(&self) -> String;
}

objekt::clone_trait_object!(Transformer);

impl fmt::Debug for Box<dyn Transformer> {
  fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
    unimplemented!()
  }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum TransformerTypes {
  Direct,
  Hiragana,
  Katakana,
  Henkan,
  Okuri,
  Abbr,
  EmEisu,
  EnKatakana,
  Yomi,
  Canceled,
  Stopped,
  SelectCandidate,
}

impl TransformerTypes {
  pub fn to_transformer(
    &self,
    config: Rc<Config>,
    dictionary: Rc<Dictionary>,
  ) -> Box<dyn Transformer> {
    match self {
      TransformerTypes::Direct => Box::new(DirectTransformer::new()),
      TransformerTypes::Henkan => Box::new(HenkanTransformer::new(
        config,
        dictionary,
        TransformerTypes::Hiragana,
      )),
      TransformerTypes::Okuri => Box::new(DirectTransformer::new()),
      TransformerTypes::Hiragana => Box::new(HiraganaTransformer::new()),
      TransformerTypes::Katakana => Box::new(DirectTransformer::new()),
      TransformerTypes::Abbr => Box::new(DirectTransformer::new()),
      TransformerTypes::EmEisu => Box::new(DirectTransformer::new()),
      TransformerTypes::EnKatakana => Box::new(DirectTransformer::new()),
      _ => unreachable!(),
    }
  }

  pub fn allow_change_transformer_to(&self) -> HashSet<TransformerTypes> {
    match self {
      TransformerTypes::Direct => set![TransformerTypes::Hiragana],
      TransformerTypes::Hiragana => set![
        TransformerTypes::Direct,
        TransformerTypes::Henkan,
        TransformerTypes::Abbr,
        TransformerTypes::Katakana,
        TransformerTypes::EnKatakana,
        TransformerTypes::EmEisu
      ],
      TransformerTypes::Henkan => set![TransformerTypes::Okuri],
      TransformerTypes::Okuri => set![],
      TransformerTypes::Katakana => set![
        TransformerTypes::Direct,
        TransformerTypes::Hiragana,
        TransformerTypes::Henkan,
        TransformerTypes::Abbr,
        TransformerTypes::EnKatakana,
        TransformerTypes::EmEisu
      ],
      TransformerTypes::EnKatakana => set![
        TransformerTypes::Direct,
        TransformerTypes::Hiragana,
        TransformerTypes::Henkan,
        TransformerTypes::Abbr,
        TransformerTypes::Katakana,
        TransformerTypes::EmEisu
      ],
      TransformerTypes::EmEisu => set![TransformerTypes::Hiragana],
      TransformerTypes::Abbr => set![],
      _ => unreachable!(),
    }
  }

  pub fn get_key_combination<'a>(&self, key_config: &'a KeyConfig) -> &'a KeyCombinations {
    match self {
      TransformerTypes::Direct => &key_config.enter_direct_transformer,
      TransformerTypes::Henkan => &key_config.enter_kanji_transformer,
      TransformerTypes::Okuri => &key_config.enter_okuri_transformer,
      TransformerTypes::Hiragana => &key_config.enter_hiragana_transformer,
      TransformerTypes::Katakana => &key_config.enter_katakana_transformer,
      TransformerTypes::EnKatakana => &key_config.enter_en_katakana_transformer,
      TransformerTypes::EmEisu => &key_config.enter_em_eisu_transformer,
      TransformerTypes::Abbr => &key_config.enter_abbr_transformer,
      _ => unreachable!(),
    }
  }
}

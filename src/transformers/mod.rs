mod direct;
mod hiragana;
mod tables;

use crate::config::KeyConfig;
use crate::keyboards::KeyCode;
use crate::set;
use std::collections::HashSet;

pub type DirectTransformer = direct::DirectTransformer;
pub type HiraganaTransformer = hiragana::HiraganaTransformer;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BufferState {
  Continue,
  Stop,
}

pub trait Transformer {
  fn is_stopped(&self) -> bool;
  fn push(&mut self, character: char);
  fn exit(&mut self) -> String;
  fn buffer_content(&self) -> String;
  fn display_string(&self) -> String;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum TransformerTypes {
  Direct,
  Hiragana,
  Katakana,
  Kanji,
  Okuri,
  Abbr,
  EmEisu,
  EnKatakana,
}

impl TransformerTypes {
  pub fn to_transformer(&self) -> Box<dyn Transformer> {
    match self {
      TransformerTypes::Direct => Box::new(DirectTransformer::new()),
      TransformerTypes::Kanji => Box::new(DirectTransformer::new()),
      TransformerTypes::Okuri => Box::new(DirectTransformer::new()),
      TransformerTypes::Hiragana => Box::new(HiraganaTransformer::new()),
      TransformerTypes::Katakana => Box::new(DirectTransformer::new()),
      TransformerTypes::Abbr => Box::new(DirectTransformer::new()),
      TransformerTypes::EmEisu => Box::new(DirectTransformer::new()),
      TransformerTypes::EnKatakana => Box::new(DirectTransformer::new()),
    }
  }

  pub fn allow_change_transformer_to(&self) -> HashSet<TransformerTypes> {
    match self {
      TransformerTypes::Direct => set![TransformerTypes::Hiragana],
      TransformerTypes::Hiragana => set![
        TransformerTypes::Direct,
        TransformerTypes::Kanji,
        TransformerTypes::Abbr,
        TransformerTypes::Katakana,
        TransformerTypes::EnKatakana,
        TransformerTypes::EmEisu
      ],
      TransformerTypes::Kanji => set![TransformerTypes::Okuri],
      TransformerTypes::Okuri => set![],
      TransformerTypes::Katakana => set![
        TransformerTypes::Direct,
        TransformerTypes::Hiragana,
        TransformerTypes::Kanji,
        TransformerTypes::Abbr,
        TransformerTypes::EnKatakana,
        TransformerTypes::EmEisu
      ],
      TransformerTypes::EnKatakana => set![
        TransformerTypes::Direct,
        TransformerTypes::Hiragana,
        TransformerTypes::Kanji,
        TransformerTypes::Abbr,
        TransformerTypes::Katakana,
        TransformerTypes::EmEisu
      ],
      TransformerTypes::EmEisu => set![TransformerTypes::Hiragana],
      TransformerTypes::Abbr => set![],
    }
  }

  pub fn get_key_combination<'a>(&self, key_config: &'a KeyConfig) -> &'a Vec<KeyCode> {
    match self {
      TransformerTypes::Direct => &key_config.enter_direct_transformer,
      TransformerTypes::Kanji => &key_config.enter_kanji_transformer,
      TransformerTypes::Okuri => &key_config.enter_okuri_transformer,
      TransformerTypes::Hiragana => &key_config.enter_hiragana_transformer,
      TransformerTypes::Katakana => &key_config.enter_katakana_transformer,
      TransformerTypes::EnKatakana => &key_config.enter_en_katakana_transformer,
      TransformerTypes::EmEisu => &key_config.enter_em_eisu_transformer,
      TransformerTypes::Abbr => &key_config.enter_abbr_transformer,
    }
  }
}

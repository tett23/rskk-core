mod aspect;
mod direct;
mod henkan;
mod hiragana;
mod tables;

use crate::keyboards::{KeyCode, KeyCombinations, KeyEvents};
use crate::{set, Dictionary, KeyConfig, RSKKConfig};
use objekt;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

pub use aspect::{
  Aspect, AspectTransformer, Canceled, SelectCandidate, Stopped, UnknownWord, Yomi,
};
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

pub trait Displayable {
  fn buffer_content(&self) -> String;
  fn display_string(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct Config {
  rskk_config: Rc<RSKKConfig>,
  dictionary: Rc<Dictionary>,
}

impl Config {
  pub fn new(rskk_config: Rc<RSKKConfig>, dictionary: Rc<Dictionary>) -> Self {
    Config {
      rskk_config,
      dictionary,
    }
  }

  pub fn rskk_config(&self) -> &RSKKConfig {
    &self.rskk_config
  }

  pub fn dictionary(&self) -> &Dictionary {
    &self.dictionary
  }

  pub fn key_config(&self) -> &KeyConfig {
    &self.rskk_config.key_config
  }
}

pub trait WithConfig {
  fn config(&self) -> Config;
}

pub trait KeyInputtable: WithConfig + objekt::Clone {
  fn push_key_event(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    event: &KeyEvents,
    last_character: Option<char>,
  ) -> Box<dyn Transformer> {
    match event {
      KeyEvents::KeyDown(key) => {
        if let Some(new_transformer_type) = self.try_change_transformer(pressing_keys) {
          let new_transformer = new_transformer_type.to_transformer(self.config());

          match new_transformer_type {
            TransformerTypes::Henkan => {
              match key.printable_key() {
                Some(character) => {
                  return new_transformer.push_character(character);
                }
                None => {}
              };
            }
            _ => {}
          };
          return new_transformer;
        };

        let new_transformer = self.push_key_code(key);
        let new_transformer = match last_character {
          Some(character) => new_transformer.push_character(character),
          None => new_transformer,
        };

        new_transformer
      }
      KeyEvents::KeyUp(_) => self.push_key_code(&KeyCode::Null),
      KeyEvents::KeyRepeat(_) => unimplemented!(),
    }
  }
  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes>;
  fn push_key_code(&self, key_code: &KeyCode) -> Box<dyn Transformer>;
  fn push_character(&self, character: char) -> Box<dyn Transformer>;
}

pub trait Transformer: TransformerState + KeyInputtable + Displayable + fmt::Debug {
  fn transformer_type(&self) -> TransformerTypes {
    unimplemented!()
  }
}

objekt::clone_trait_object!(Transformer);

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
  UnknownWord,
}

impl TransformerTypes {
  pub fn to_transformer(&self, config: Config) -> Box<dyn Transformer> {
    match self {
      TransformerTypes::Direct => Box::new(DirectTransformer::new(config)),
      TransformerTypes::Henkan => {
        Box::new(HenkanTransformer::new(config, TransformerTypes::Hiragana))
      }
      TransformerTypes::Okuri => Box::new(DirectTransformer::new(config)),
      TransformerTypes::Hiragana => Box::new(HiraganaTransformer::new(config)),
      TransformerTypes::Katakana => Box::new(DirectTransformer::new(config)),
      TransformerTypes::Abbr => Box::new(DirectTransformer::new(config)),
      TransformerTypes::EmEisu => Box::new(DirectTransformer::new(config)),
      TransformerTypes::EnKatakana => Box::new(DirectTransformer::new(config)),
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
      TransformerTypes::Henkan => &key_config.enter_henkan_transformer,
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

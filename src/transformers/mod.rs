mod aspect;
mod direct;
mod henkan;
mod hiragana;
mod stackable;
mod tables;

use crate::keyboards::{KeyCode, KeyCombinations, KeyEvents, MetaKey};
use crate::{set, Dictionary, KeyConfig, RSKKConfig};
use objekt;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

pub use aspect::{
  Aspect, AspectTransformer, Canceled, ContinuousTransformer, SelectCandidate, Stopped,
  UnknownWord, Word, YomiTransformer,
};
pub use direct::DirectTransformer;
pub use henkan::HenkanTransformer;
pub use hiragana::HiraganaTransformer;
pub use stackable::Stackable;

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

pub trait Transformable:
  AsTransformerTrait + WithConfig + TransformerState + Displayable + fmt::Debug + objekt::Clone
{
  fn transformer_type(&self) -> TransformerTypes {
    unimplemented!()
  }

  fn push_key_event(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    event: &KeyEvents,
    last_character: Option<char>,
  ) -> Box<dyn Transformable> {
    match event {
      KeyEvents::KeyDown(key) => self.key_down(pressing_keys, key, last_character),
      KeyEvents::KeyUp(_) => self.key_up(),
      KeyEvents::KeyRepeat(_) => unimplemented!(),
    }
  }

  fn key_down(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    key: &KeyCode,
    last_character: Option<char>,
  ) -> Box<dyn Transformable> {
    println!(
      "change transformer start {:?} {:?}",
      key,
      self.transformer_type()
    );
    if let Some(new_transformer) = self.try_change_transformer(pressing_keys, key) {
      return new_transformer;
    };

    println!(
      "change transformer try_change_transformer {:?}",
      self.transformer_type()
    );
    let new_transformer = self.push_meta_key(key);
    println!(
      "change transformer push_meta key {:?}",
      new_transformer.transformer_type()
    );
    let new_transformer = match last_character {
      Some(character) => new_transformer.push_character(character),
      None => new_transformer,
    };
    println!(
      "change transformer push_character {:?}",
      new_transformer.transformer_type()
    );
    println!();

    new_transformer
  }

  fn key_up(&self) -> Box<dyn Transformable> {
    self.push_meta_key(&KeyCode::Null)
  }
  fn try_change_transformer(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>>;
  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformable> {
    let target = self.send_target();

    let new_transformer = match key_code {
      KeyCode::Meta(MetaKey::Escape) => target.push_escape(),
      KeyCode::PrintableMeta(MetaKey::Enter, _) | KeyCode::Meta(MetaKey::Enter) => {
        target.push_enter()
      }
      KeyCode::PrintableMeta(MetaKey::Space, _) | KeyCode::Meta(MetaKey::Space) => {
        target.push_space()
      }
      KeyCode::PrintableMeta(MetaKey::Backspace, _) | KeyCode::Meta(MetaKey::Backspace) => {
        target.push_backspace()
      }
      KeyCode::PrintableMeta(MetaKey::Delete, _) | KeyCode::Meta(MetaKey::Delete) => {
        target.push_delete()
      }
      KeyCode::PrintableMeta(MetaKey::Tab, _) | KeyCode::Meta(MetaKey::Tab) => target.push_tab(),
      KeyCode::Meta(MetaKey::ArrowRight) => target.push_arrow_right(),
      KeyCode::Meta(MetaKey::ArrowDown) => target.push_arrow_down(),
      KeyCode::Meta(MetaKey::ArrowLeft) => target.push_arrow_left(),
      KeyCode::Meta(MetaKey::ArrowUp) => target.push_arrow_up(),
      _ => return self.push_any_character(&target, key_code),
    };

    self.transformer_updated(new_transformer)
  }
  fn transformer_updated(&self, new_transformer: Box<dyn Transformable>) -> Box<dyn Transformable> {
    new_transformer
  }
  fn push_character(&self, character: char) -> Box<dyn Transformable>;

  fn push_escape(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_enter(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_space(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_backspace(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_delete(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_tab(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_null(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_arrow_right(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_arrow_down(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_arrow_left(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_arrow_up(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
  fn push_any_character(&self, _: &Box<dyn Transformable>, _: &KeyCode) -> Box<dyn Transformable> {
    self.as_trait()
  }
}

pub trait AsTransformerTrait {
  fn as_trait(&self) -> Box<dyn Transformable>;

  fn send_target(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
}

objekt::clone_trait_object!(Transformable);

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
  ContinuousTransformer,
}

impl TransformerTypes {
  pub fn to_transformer(&self, config: Config) -> Box<dyn Transformable> {
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
      TransformerTypes::ContinuousTransformer => Box::new(ContinuousTransformer::new(
        config,
        TransformerTypes::Hiragana,
      )),
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

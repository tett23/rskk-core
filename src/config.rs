use crate::keyboards::keycodes::KeyCode::*;
use crate::keyboards::{KeyCode, Keyboards};
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {
  pub keyboard_type: Keyboards,
  pub key_config: Rc<KeyConfig>,
  pub is_enable_sticky_shift: bool,
}

impl Config {
  pub fn default_config() -> Self {
    Config {
      keyboard_type: Keyboards::US,
      key_config: Rc::new(KeyConfig::default_config()),
      is_enable_sticky_shift: false,
    }
  }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct KeyConfig {
  pub enter: Vec<KeyCode>,
  pub enter_kanji_transformer: Vec<KeyCode>,
  pub enter_okuri_transformer: Vec<KeyCode>,
  pub enter_hiragana_transformer: Vec<KeyCode>,
  pub enter_katakana_transformer: Vec<KeyCode>,
  pub enter_en_katakana_transformer: Vec<KeyCode>,
  pub enter_em_eisu_transformer: Vec<KeyCode>,
  pub enter_abbr_transformer: Vec<KeyCode>,
  pub enter_direct_transformer: Vec<KeyCode>,
  pub sticky_key: Vec<KeyCode>,
}

impl KeyConfig {
  pub fn default_config() -> Self {
    KeyConfig {
      enter: vec![Enter],
      enter_kanji_transformer: vec![Shift],
      enter_okuri_transformer: vec![Shift],
      enter_hiragana_transformer: vec![Ctrl, KeyJ],
      enter_katakana_transformer: vec![KeyQ],
      enter_en_katakana_transformer: vec![Ctrl, KeyQ],
      enter_em_eisu_transformer: vec![Shift, KeyL],
      enter_abbr_transformer: vec![Slash],
      enter_direct_transformer: vec![KeyL],
      sticky_key: vec![Semicolon],
    }
  }
}

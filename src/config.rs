use crate::keyboards::keycodes::KeyCode::*;
use crate::keyboards::{KeyCode, Keyboards};
use crate::set;
use std::collections::HashSet;
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
  pub enter: HashSet<KeyCode>,
  pub enter_kanji_transformer: HashSet<KeyCode>,
  pub enter_okuri_transformer: HashSet<KeyCode>,
  pub enter_hiragana_transformer: HashSet<KeyCode>,
  pub enter_katakana_transformer: HashSet<KeyCode>,
  pub enter_en_katakana_transformer: HashSet<KeyCode>,
  pub enter_em_eisu_transformer: HashSet<KeyCode>,
  pub enter_abbr_transformer: HashSet<KeyCode>,
  pub enter_direct_transformer: HashSet<KeyCode>,
  pub sticky_key: HashSet<KeyCode>,
}

impl KeyConfig {
  pub fn default_config() -> Self {
    KeyConfig {
      enter: set![Enter],
      enter_kanji_transformer: set![Shift],
      enter_okuri_transformer: set![Shift],
      enter_hiragana_transformer: set![Ctrl, KeyJ],
      enter_katakana_transformer: set![KeyQ],
      enter_en_katakana_transformer: set![Ctrl, KeyQ],
      enter_em_eisu_transformer: set![Shift, KeyL],
      enter_abbr_transformer: set![Slash],
      enter_direct_transformer: set![KeyL],
      sticky_key: set![Semicolon],
    }
  }
}

use crate::keyboards::{KeyCode, KeyCombination, KeyCombinations, Keyboards};
use crate::{combo, combos};
use std::collections::HashSet;
use std::rc::Rc;
use KeyCode::*;

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
  pub enter: KeyCombinations,
  pub enter_kanji_transformer: KeyCombinations,
  pub enter_okuri_transformer: KeyCombinations,
  pub enter_hiragana_transformer: KeyCombinations,
  pub enter_katakana_transformer: KeyCombinations,
  pub enter_en_katakana_transformer: KeyCombinations,
  pub enter_em_eisu_transformer: KeyCombinations,
  pub enter_abbr_transformer: KeyCombinations,
  pub enter_direct_transformer: KeyCombinations,
  pub sticky_key: KeyCombinations,
}

impl KeyConfig {
  pub fn default_config() -> Self {
    KeyConfig {
      enter: combos![combo![KeyA]],
      enter_kanji_transformer: combos![
        combo![Shift, KeyA],
        combo![Shift, KeyB],
        combo![Shift, KeyC],
        combo![Shift, KeyD],
        combo![Shift, KeyE],
        combo![Shift, KeyF],
        combo![Shift, KeyG],
        combo![Shift, KeyH],
        combo![Shift, KeyI],
        combo![Shift, KeyJ],
        combo![Shift, KeyK],
        combo![Shift, KeyL],
        combo![Shift, KeyM],
        combo![Shift, KeyN],
        combo![Shift, KeyO],
        combo![Shift, KeyP],
        combo![Shift, KeyQ],
        combo![Shift, KeyR],
        combo![Shift, KeyS],
        combo![Shift, KeyT],
        combo![Shift, KeyU],
        combo![Shift, KeyW],
        combo![Shift, KeyX],
        combo![Shift, KeyY],
        combo![Shift, KeyZ]
      ],
      enter_okuri_transformer: combos![
        combo![Shift, KeyA],
        combo![Shift, KeyB],
        combo![Shift, KeyC],
        combo![Shift, KeyD],
        combo![Shift, KeyE],
        combo![Shift, KeyF],
        combo![Shift, KeyG],
        combo![Shift, KeyH],
        combo![Shift, KeyI],
        combo![Shift, KeyJ],
        combo![Shift, KeyK],
        combo![Shift, KeyL],
        combo![Shift, KeyM],
        combo![Shift, KeyN],
        combo![Shift, KeyO],
        combo![Shift, KeyP],
        combo![Shift, KeyQ],
        combo![Shift, KeyR],
        combo![Shift, KeyS],
        combo![Shift, KeyT],
        combo![Shift, KeyU],
        combo![Shift, KeyW],
        combo![Shift, KeyX],
        combo![Shift, KeyY],
        combo![Shift, KeyZ]
      ],
      enter_hiragana_transformer: combos![combo![Ctrl, KeyJ]],
      enter_katakana_transformer: combos![combo![KeyQ]],
      enter_en_katakana_transformer: combos![combo![Ctrl, KeyQ]],
      enter_em_eisu_transformer: combos![combo![Shift, KeyL]],
      enter_abbr_transformer: combos![combo![Slash]],
      enter_direct_transformer: combos![combo![KeyL]],
      sticky_key: combos![combo![Semicolon]],
    }
  }
}

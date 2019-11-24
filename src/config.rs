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
      enter: combos![combo![Enter]],
      enter_kanji_transformer: combos![
        combo![Shift, Printable('a')],
        combo![Shift, Printable('b')],
        combo![Shift, Printable('c')],
        combo![Shift, Printable('d')],
        combo![Shift, Printable('e')],
        combo![Shift, Printable('f')],
        combo![Shift, Printable('g')],
        combo![Shift, Printable('h')],
        combo![Shift, Printable('i')],
        combo![Shift, Printable('j')],
        combo![Shift, Printable('k')],
        combo![Shift, Printable('l')],
        combo![Shift, Printable('m')],
        combo![Shift, Printable('n')],
        combo![Shift, Printable('o')],
        combo![Shift, Printable('p')],
        combo![Shift, Printable('q')],
        combo![Shift, Printable('r')],
        combo![Shift, Printable('s')],
        combo![Shift, Printable('t')],
        combo![Shift, Printable('u')],
        combo![Shift, Printable('w')],
        combo![Shift, Printable('x')],
        combo![Shift, Printable('y')],
        combo![Shift, Printable('z')]
      ],
      enter_okuri_transformer: combos![
        combo![Shift, Printable('a')],
        combo![Shift, Printable('b')],
        combo![Shift, Printable('c')],
        combo![Shift, Printable('d')],
        combo![Shift, Printable('e')],
        combo![Shift, Printable('f')],
        combo![Shift, Printable('g')],
        combo![Shift, Printable('h')],
        combo![Shift, Printable('i')],
        combo![Shift, Printable('j')],
        combo![Shift, Printable('k')],
        combo![Shift, Printable('l')],
        combo![Shift, Printable('m')],
        combo![Shift, Printable('n')],
        combo![Shift, Printable('o')],
        combo![Shift, Printable('p')],
        combo![Shift, Printable('q')],
        combo![Shift, Printable('r')],
        combo![Shift, Printable('s')],
        combo![Shift, Printable('t')],
        combo![Shift, Printable('u')],
        combo![Shift, Printable('w')],
        combo![Shift, Printable('x')],
        combo![Shift, Printable('y')],
        combo![Shift, Printable('z')]
      ],
      enter_hiragana_transformer: combos![combo![Ctrl, Printable('j')]],
      enter_katakana_transformer: combos![combo![Printable('q')]],
      enter_en_katakana_transformer: combos![combo![Ctrl, Printable('q')]],
      enter_em_eisu_transformer: combos![combo![Shift, Printable('l')]],
      enter_abbr_transformer: combos![combo![Printable('/')]],
      enter_direct_transformer: combos![combo![Printable('l')]],
      sticky_key: combos![combo![Printable(';')]],
    }
  }
}

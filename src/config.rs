use crate::keyboards::{KeyCombination, KeyCombinations, Keyboards};
use crate::{combo, combos, key};
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
      enter: combos![combo![key!("enter")]],
      enter_kanji_transformer: combos![
        combo![key!("shift"), key!("a")],
        combo![key!("shift"), key!("b")],
        combo![key!("shift"), key!("c")],
        combo![key!("shift"), key!("d")],
        combo![key!("shift"), key!("e")],
        combo![key!("shift"), key!("f")],
        combo![key!("shift"), key!("g")],
        combo![key!("shift"), key!("h")],
        combo![key!("shift"), key!("i")],
        combo![key!("shift"), key!("j")],
        combo![key!("shift"), key!("k")],
        combo![key!("shift"), key!("l")],
        combo![key!("shift"), key!("m")],
        combo![key!("shift"), key!("n")],
        combo![key!("shift"), key!("o")],
        combo![key!("shift"), key!("p")],
        combo![key!("shift"), key!("q")],
        combo![key!("shift"), key!("r")],
        combo![key!("shift"), key!("s")],
        combo![key!("shift"), key!("t")],
        combo![key!("shift"), key!("u")],
        combo![key!("shift"), key!("w")],
        combo![key!("shift"), key!("x")],
        combo![key!("shift"), key!("y")],
        combo![key!("shift"), key!("z")]
      ],
      enter_okuri_transformer: combos![
        combo![key!("shift"), key!("a")],
        combo![key!("shift"), key!("b")],
        combo![key!("shift"), key!("c")],
        combo![key!("shift"), key!("d")],
        combo![key!("shift"), key!("e")],
        combo![key!("shift"), key!("f")],
        combo![key!("shift"), key!("g")],
        combo![key!("shift"), key!("h")],
        combo![key!("shift"), key!("i")],
        combo![key!("shift"), key!("j")],
        combo![key!("shift"), key!("k")],
        combo![key!("shift"), key!("l")],
        combo![key!("shift"), key!("m")],
        combo![key!("shift"), key!("n")],
        combo![key!("shift"), key!("o")],
        combo![key!("shift"), key!("p")],
        combo![key!("shift"), key!("q")],
        combo![key!("shift"), key!("r")],
        combo![key!("shift"), key!("s")],
        combo![key!("shift"), key!("t")],
        combo![key!("shift"), key!("u")],
        combo![key!("shift"), key!("w")],
        combo![key!("shift"), key!("x")],
        combo![key!("shift"), key!("y")],
        combo![key!("shift"), key!("z")]
      ],
      enter_hiragana_transformer: combos![combo![key!("ctrl"), key!("j")]],
      enter_katakana_transformer: combos![combo![key!("q")]],
      enter_en_katakana_transformer: combos![combo![key!("ctrl"), key!("q")]],
      enter_em_eisu_transformer: combos![combo![key!("shift"), key!("l")]],
      enter_abbr_transformer: combos![combo![key!("/")]],
      enter_direct_transformer: combos![combo![key!("l")]],
      sticky_key: combos![combo![key!(";")]],
    }
  }
}

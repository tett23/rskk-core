use crate::keyboards::{KeyCode, KeyCombination, KeyCombinations, Keyboards};
use crate::transformers::TransformerTypes;
use crate::{combo, combos, key};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct RSKKConfig {
  pub keyboard_type: Keyboards,
  pub key_config: Rc<KeyConfig>,
  pub is_enable_sticky_shift: bool,
}

impl RSKKConfig {
  pub fn default_config() -> Self {
    RSKKConfig {
      keyboard_type: Keyboards::US,
      key_config: Rc::new(KeyConfig::default_config()),
      is_enable_sticky_shift: false,
    }
  }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct KeyConfig {
  pub enter: KeyCombinations,
  pub enter_henkan_transformer: KeyCombinations,
  pub enter_okuri_transformer: KeyCombinations,
  pub enter_hiragana_transformer: KeyCombinations,
  pub enter_katakana_transformer: KeyCombinations,
  pub enter_en_katakana_transformer: KeyCombinations,
  pub enter_em_eisu_transformer: KeyCombinations,
  pub enter_abbr_transformer: KeyCombinations,
  pub enter_direct_transformer: KeyCombinations,
  pub sticky_key: KeyCombinations,
}

impl KeyConfig {}

impl KeyConfig {
  pub fn try_change_transformer(
    &self,
    allow: &HashSet<TransformerTypes>,
    pressing_keys: &HashSet<KeyCode>,
  ) -> Option<TransformerTypes> {
    if allow.contains(&TransformerTypes::Henkan)
      && self.enter_henkan_transformer.fulfilled(pressing_keys)
    {
      return Some(TransformerTypes::Henkan);
    }
    if allow.contains(&TransformerTypes::Okuri)
      && self.enter_okuri_transformer.fulfilled(pressing_keys)
    {
      return Some(TransformerTypes::Okuri);
    }
    if allow.contains(&TransformerTypes::Hiragana)
      && self.enter_hiragana_transformer.fulfilled(pressing_keys)
    {
      return Some(TransformerTypes::Hiragana);
    }
    if allow.contains(&TransformerTypes::EnKatakana)
      && self.enter_en_katakana_transformer.fulfilled(pressing_keys)
    {
      return Some(TransformerTypes::EnKatakana);
    }
    if allow.contains(&TransformerTypes::EmEisu)
      && self.enter_em_eisu_transformer.fulfilled(pressing_keys)
    {
      return Some(TransformerTypes::EmEisu);
    }
    if allow.contains(&TransformerTypes::Abbr)
      && self.enter_abbr_transformer.fulfilled(pressing_keys)
    {
      return Some(TransformerTypes::Abbr);
    }
    if allow.contains(&TransformerTypes::Direct)
      && self.enter_direct_transformer.fulfilled(pressing_keys)
    {
      return Some(TransformerTypes::Direct);
    }

    None
  }

  pub fn default_config() -> Self {
    KeyConfig {
      enter: combos![combo![key!("enter")]],
      enter_henkan_transformer: combos![
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

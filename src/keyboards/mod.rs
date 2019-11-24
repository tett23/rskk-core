pub mod keycodes;
pub mod us;

use crate::config::KeyConfig;
use crate::TransformerTypes;
use std::collections::HashSet;

pub use keycodes::{KeyCode, KeyCombination, KeyCombinations};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Keyboards {
  US,
}

impl Keyboards {
  pub fn to_keyboard(&self) -> Box<dyn Keyboard> {
    match self {
      Keyboards::US => Box::new(us::US::new()),
    }
  }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum KeyEvents {
  KeyDown(KeyCode),
  KeyRepeat(KeyCode),
  KeyUp(KeyCode),
}

pub trait Keyboard {
  fn key_down(&mut self, key: &KeyCode);
  fn key_up(&mut self, key: &KeyCode);
  fn pressing_keys(&self) -> &HashSet<KeyCode>;
  fn last_character(&self) -> Option<char>;

  fn push_event(&mut self, event: &KeyEvents) {
    match event {
      KeyEvents::KeyDown(key) => self.key_down(key),
      KeyEvents::KeyUp(key) => self.key_up(key),
      KeyEvents::KeyRepeat(_) => unimplemented!(),
    }
  }

  fn push_events(&mut self, events: &Vec<KeyEvents>) {
    events.iter().for_each(|e| self.push_event(e))
  }

  fn is_pressing_shift(&self) -> bool {
    self.is_pressing(&KeyCode::Shift)
  }

  fn is_pressing_ctrl(&self) -> bool {
    self.is_pressing(&KeyCode::Ctrl)
  }

  fn is_pressing(&self, key: &KeyCode) -> bool {
    self.pressing_keys().contains(key)
  }

  fn try_change_transformer(
    &self,
    key_config: &KeyConfig,
    current_transformer: &TransformerTypes,
  ) -> Option<TransformerTypes> {
    if let Some(ret) = current_transformer
      .allow_change_transformer_to()
      .iter()
      .find(|&&transformer| {
        transformer
          .get_key_combination(key_config)
          .fulfilled(&self.pressing_keys())
      })
    {
      Some(ret.clone())
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::config::KeyConfig;
  use crate::tests::helpers::str_to_key_code_vector;

  #[test]
  fn change_transformer() {
    let key_config = KeyConfig::default_config();

    let mut keyboard = us::US::new();
    keyboard.push_events(&str_to_key_code_vector("a"));
    assert_eq!(
      None,
      keyboard.try_change_transformer(&key_config, &TransformerTypes::Direct)
    );

    let mut keyboard = us::US::new();
    keyboard.push_events(&str_to_key_code_vector("[down:ctrl][down:j]"));
    assert_eq!(
      Some(TransformerTypes::Hiragana),
      keyboard.try_change_transformer(&key_config, &TransformerTypes::Direct)
    );

    let mut keyboard = us::US::new();
    keyboard.push_events(&str_to_key_code_vector("[down:l]"));
    assert_eq!(
      Some(TransformerTypes::Direct),
      keyboard.try_change_transformer(&key_config, &TransformerTypes::Hiragana)
    );
  }
}

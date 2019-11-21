pub mod keycodes;
pub mod us;

use crate::config::KeyConfig;
use crate::TransformerTypes;
use std::collections::HashSet;

pub type KeyCode = keycodes::KeyCode;

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

pub trait Keyboard {
  fn key_down(&mut self, key: &KeyCode) -> Option<char>;
  fn key_up(&mut self, key: &KeyCode);
  fn pressing_keys(&self) -> &HashSet<KeyCode>;
  fn last_character(&self) -> Option<char>;

  fn repeat(&self) -> Option<char> {
    self.last_character()
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
    transformer_type: &TransformerTypes,
  ) -> Option<TransformerTypes> {
    if let Some(ret) = transformer_type
      .allow_change_transformer_to()
      .iter()
      .find(|&&transformer| {
        transformer
          .get_key_combination(key_config)
          .iter()
          .all(|&k| self.is_pressing(&k))
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
  use crate::keyboards::keycodes::KeyCode::*;

  #[test]
  fn change_transformer() {
    let key_config = KeyConfig::default_config();

    let mut keyboard = us::US::new();
    keyboard.key_down(&KeyA);
    assert_eq!(
      None,
      keyboard.try_change_transformer(&key_config, &TransformerTypes::Direct)
    );

    let mut keyboard = us::US::new();
    keyboard.key_down(&Ctrl);
    keyboard.key_down(&KeyJ);
    assert_eq!(
      Some(TransformerTypes::Hiragana),
      keyboard.try_change_transformer(&key_config, &TransformerTypes::Direct)
    );

    let mut keyboard = us::US::new();
    keyboard.key_down(&KeyL);
    assert_eq!(
      Some(TransformerTypes::Direct),
      keyboard.try_change_transformer(&key_config, &TransformerTypes::Hiragana)
    );
  }
}

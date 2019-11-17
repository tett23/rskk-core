mod us;

use crate::keycodes::KeyCode;
use std::collections::HashSet;

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
}

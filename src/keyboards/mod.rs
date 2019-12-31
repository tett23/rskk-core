pub mod keycodes;
pub mod us;

use objekt;
use std::collections::HashSet;
use std::convert::TryFrom;

pub use keycodes::{KeyCode, KeyCombination, KeyCombinations, MetaKey};

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

impl TryFrom<(u16, u16)> for KeyEvents {
  type Error = &'static str;

  fn try_from(pair: (u16, u16)) -> Result<Self, Self::Error> {
    dbg!("try_from", &pair);

    let (event_type, code) = pair;
    KeyCode::try_from(code)
      .map(|code| match event_type {
        1 => Ok(KeyEvents::KeyDown(code)),
        2 => Ok(KeyEvents::KeyUp(code)),
        _ => Err(""),
      })
      .unwrap_or(Err(""))
  }
}

pub trait Keyboard: objekt::Clone + Send + Sync {
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
    self.is_pressing(&KeyCode::Meta(MetaKey::Shift))
  }

  fn is_pressing_ctrl(&self) -> bool {
    self.is_pressing(&KeyCode::Meta(MetaKey::Ctrl))
  }

  fn is_pressing(&self, key: &KeyCode) -> bool {
    self.pressing_keys().contains(key)
  }
}

objekt::clone_trait_object!(Keyboard);

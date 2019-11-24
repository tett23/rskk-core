use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum KeyCode {
  KeyA,
  KeyB,
  KeyC,
  KeyD,
  KeyE,
  KeyF,
  KeyG,
  KeyH,
  KeyI,
  KeyJ,
  KeyK,
  KeyL,
  KeyM,
  KeyN,
  KeyO,
  KeyP,
  KeyQ,
  KeyR,
  KeyS,
  KeyT,
  KeyU,
  KeyW,
  KeyX,
  KeyY,
  KeyZ,
  Key1,
  Key2,
  Key3,
  Key4,
  Key5,
  Key6,
  Key7,
  Key8,
  Key9,
  Key0,
  Semicolon,
  Slash,
  Ctrl,
  Shift,
  Alt,
  Super,
  Enter,
  Space,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct KeyCombination(HashSet<KeyCode>);

impl KeyCombination {
  pub fn new(set: HashSet<KeyCode>) -> Self {
    KeyCombination(set)
  }

  pub fn fulfilled(&self, pressed: &HashSet<KeyCode>) -> bool {
    self.0.iter().all(|&k| pressed.iter().any(|&kk| k == kk))
  }
}

impl Hash for KeyCombination {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.iter().for_each(|v| {
      v.hash(state);
    });

    state.finish();
  }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct KeyCombinations(HashSet<KeyCombination>);

impl KeyCombinations {
  pub fn new(set: HashSet<KeyCombination>) -> Self {
    KeyCombinations(set)
  }

  pub fn fulfilled(&self, pressed: &HashSet<KeyCode>) -> bool {
    self
      .0
      .iter()
      .any(|combination| combination.fulfilled(pressed))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::set;
  use crate::{combo, combos};
  use KeyCode::*;

  mod key_combination {
    use super::*;

    #[test]
    fn fulfilled() {
      let combination = combo![Ctrl, KeyJ];

      assert!(combination.fulfilled(&set![Ctrl, KeyJ]));
      assert!(!combination.fulfilled(&set![KeyA]));
    }
  }

  mod key_combinations {
    use super::*;

    #[test]

    fn fulfilled() {
      let combination = combos![combo![KeyA], combo![Ctrl, KeyJ]];

      assert!(combination.fulfilled(&set![KeyA]));
      assert!(combination.fulfilled(&set![Ctrl, KeyJ]));
      assert!(!combination.fulfilled(&set![KeyB]));
    }
  }
}

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum KeyCode {
  // FIXME metaかつprintableなキーがある。enterとか
  Null,
  Printable(char),
  Ctrl,
  Shift,
  Alt,
  Super,
  Enter,
  Space,
  Tab,
  Escape,
  Delete,
  Backspace,
  ArrowRight,
  ArrowDown,
  ArrowLeft,
  ArrowUp,
}

impl KeyCode {
  pub fn is_printable(&self) -> bool {
    match self {
      KeyCode::Printable(_) => true,
      _ => false,
    }
  }
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
  use crate::key;
  use crate::set;
  use crate::{combo, combos};

  mod key_combination {
    use super::*;

    #[test]
    fn fulfilled() {
      let combination = combo![key!("ctrl"), key!("j")];

      assert!(combination.fulfilled(&set![key!("ctrl"), key!("j")]));
      assert!(!combination.fulfilled(&set![key!("a")]));
    }
  }

  mod key_combinations {
    use super::*;

    #[test]

    fn fulfilled() {
      let combination = combos![combo![key!("a")], combo![key!("ctrl"), key!("j")]];

      assert!(combination.fulfilled(&set![key!("a")]));
      assert!(combination.fulfilled(&set![key!("ctrl"), key!("j")]));
      assert!(!combination.fulfilled(&set![key!("b")]));
    }
  }
}

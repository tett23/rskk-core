use crate::key;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum KeyCode {
  Null,
  Meta(MetaKey),
  Printable(char),
  PrintableMeta(MetaKey, char),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum MetaKey {
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
  pub fn printable_key(&self) -> Option<char> {
    match self {
      KeyCode::Printable(character) => Some(*character),
      KeyCode::PrintableMeta(_, character) => Some(*character),
      _ => None,
    }
  }

  pub fn is_printable(&self) -> bool {
    match self {
      KeyCode::Printable(_) => true,
      KeyCode::PrintableMeta(_, _) => true,
      _ => false,
    }
  }
}

impl TryFrom<u16> for KeyCode {
  type Error = &'static str;

  fn try_from(code: u16) -> Result<Self, Self::Error> {
    match code {
      0 => Ok(key!("a")),
      11 => Ok(key!("b")),
      8 => Ok(key!("c")),
      2 => Ok(key!("d")),
      14 => Ok(key!("e")),
      3 => Ok(key!("f")),
      5 => Ok(key!("g")),
      4 => Ok(key!("h")),
      34 => Ok(key!("i")),
      38 => Ok(key!("j")),
      40 => Ok(key!("k")),
      37 => Ok(key!("l")),
      46 => Ok(key!("m")),
      45 => Ok(key!("n")),
      31 => Ok(key!("o")),
      35 => Ok(key!("p")),
      12 => Ok(key!("q")),
      15 => Ok(key!("r")),
      1 => Ok(key!("s")),
      17 => Ok(key!("t")),
      32 => Ok(key!("u")),
      9 => Ok(key!("v")),
      13 => Ok(key!("w")),
      7 => Ok(key!("x")),
      16 => Ok(key!("y")),
      6 => Ok(key!("z")),
      29 => Ok(key!("0")),
      18 => Ok(key!("1")),
      19 => Ok(key!("2")),
      20 => Ok(key!("3")),
      21 => Ok(key!("4")),
      23 => Ok(key!("5")),
      22 => Ok(key!("6")),
      26 => Ok(key!("7")),
      28 => Ok(key!("8")),
      25 => Ok(key!("9")),
      27 => Ok(key!("–")),
      24 => Ok(key!("+")),
      33 => Ok(key!("[")),
      30 => Ok(key!("]")),
      42 => Ok(key!("\\")),
      41 => Ok(key!(";")),
      39 => Ok(key!("‘")),
      43 => Ok(key!(",")),
      47 => Ok(key!(".")),
      44 => Ok(key!("/")),
      122 => Ok(key!("F1")),
      120 => Ok(key!("F2")),
      99 => Ok(key!("F3")),
      118 => Ok(key!("F4")),
      96 => Ok(key!("F5")),
      97 => Ok(key!("F6")),
      98 => Ok(key!("F7")),
      100 => Ok(key!("F8")),
      101 => Ok(key!("F9")),
      109 => Ok(key!("F10")),
      103 => Ok(key!("F11")),
      111 => Ok(key!("F12")),
      105 => Ok(key!("F13")),
      107 => Ok(key!("F14")),
      113 => Ok(key!("F15")),
      106 => Ok(key!("F16")),
      64 => Ok(key!("F17")),
      79 => Ok(key!("F18")),
      80 => Ok(key!("F19")),
      48 => Ok(key!("tab")),
      57 => Ok(key!("caps_lock")),
      49 => Ok(key!("space")),
      52 => Ok(key!("return")),
      56 => Ok(key!("left_shift")),
      60 => Ok(key!("right_shift")),
      58 => Ok(key!("left_option")),
      61 => Ok(key!("right_option")),
      59 => Ok(key!("left_control")),
      62 => Ok(key!("right_control")),
      55 => Ok(key!("left_command")),
      54 => Ok(key!("right_command")),
      51 => Ok(key!("delete")),
      53 => Ok(key!("escape")),
      123 => Ok(key!("arrow_left")),
      124 => Ok(key!("arrow_right")),
      126 => Ok(key!("arrow_up")),
      125 => Ok(key!("arrow_down")),
      63 => Ok(key!("fn")),
      116 => Ok(key!("home")),
      121 => Ok(key!("end")),
      115 => Ok(key!("page_up")),
      119 => Ok(key!("page_down")),
      71 => Ok(key!("clear")),
      81 => Ok(key!("=")),
      75 => Ok(key!("/")),
      67 => Ok(key!("*")),
      78 => Ok(key!("–")),
      69 => Ok(key!("+")),
      76 => Ok(key!("enter")),
      82 => Ok(key!("0")),
      83 => Ok(key!("1")),
      84 => Ok(key!("2")),
      85 => Ok(key!("3")),
      86 => Ok(key!("4")),
      87 => Ok(key!("5")),
      88 => Ok(key!("6")),
      89 => Ok(key!("7")),
      91 => Ok(key!("8")),
      92 => Ok(key!("9")),
      _ => Err(""),
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

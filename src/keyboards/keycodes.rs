use crate::key;
use std::collections::HashSet;
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

impl From<u16> for KeyCode {
  fn from(code: u16) -> Self {
    match code {
      0 => key!("a"),
      11 => key!("b"),
      8 => key!("c"),
      2 => key!("d"),
      14 => key!("e"),
      3 => key!("f"),
      5 => key!("g"),
      4 => key!("h"),
      34 => key!("i"),
      38 => key!("j"),
      40 => key!("k"),
      37 => key!("l"),
      46 => key!("m"),
      45 => key!("n"),
      31 => key!("o"),
      35 => key!("p"),
      12 => key!("q"),
      15 => key!("r"),
      1 => key!("s"),
      17 => key!("t"),
      32 => key!("u"),
      9 => key!("v"),
      13 => key!("w"),
      7 => key!("x"),
      16 => key!("y"),
      6 => key!("z"),
      29 => key!("0"),
      18 => key!("1"),
      19 => key!("2"),
      20 => key!("3"),
      21 => key!("4"),
      23 => key!("5"),
      22 => key!("6"),
      26 => key!("7"),
      28 => key!("8"),
      25 => key!("9"),
      27 => key!("–"),
      24 => key!("+"),
      33 => key!("["),
      30 => key!("]"),
      42 => key!("\\"),
      41 => key!(";"),
      39 => key!("‘"),
      43 => key!(","),
      47 => key!("."),
      44 => key!("/"),
      122 => key!("F1"),
      120 => key!("F2"),
      99 => key!("F3"),
      118 => key!("F4"),
      96 => key!("F5"),
      97 => key!("F6"),
      98 => key!("F7"),
      100 => key!("F8"),
      101 => key!("F9"),
      109 => key!("F10"),
      103 => key!("F11"),
      111 => key!("F12"),
      105 => key!("F13"),
      107 => key!("F14"),
      113 => key!("F15"),
      106 => key!("F16"),
      64 => key!("F17"),
      79 => key!("F18"),
      80 => key!("F19"),
      48 => key!("tab"),
      57 => key!("caps lock"),
      49 => key!("space"),
      52 => key!("return"),
      56 => key!("left shift"),
      60 => key!("right shift"),
      58 => key!("left option"),
      61 => key!("right option"),
      59 => key!("left control"),
      62 => key!("right control"),
      55 => key!("left command"),
      54 => key!("right command"),
      51 => key!("delete"),
      53 => key!("esc"),
      123 => key!("left arrow"),
      124 => key!("right arrow"),
      126 => key!("up arrow"),
      125 => key!("down arrow"),
      63 => key!("fn"),
      116 => key!("home"),
      121 => key!("end"),
      115 => key!("page up"),
      119 => key!("page down"),
      71 => key!("clear"),
      81 => key!("="),
      75 => key!("/"),
      67 => key!("*"),
      78 => key!("–"),
      69 => key!("+"),
      76 => key!("enter"),
      82 => key!("0"),
      83 => key!("1"),
      84 => key!("2"),
      85 => key!("3"),
      86 => key!("4"),
      87 => key!("5"),
      88 => key!("6"),
      89 => key!("7"),
      91 => key!("8"),
      92 => key!("9"),
      _ => unreachable!(),
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

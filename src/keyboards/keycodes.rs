use std::collections::HashSet;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Serialize, Deserialize)]
pub enum KeyCode {
  Null,
  Meta(MetaKey),
  Printable(char),
  PrintableMeta(MetaKey, char),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Serialize, Deserialize)]
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

impl From<&str> for KeyCode {
  fn from(name: &str) -> KeyCode {
    match name {
      "ctrl" | "left_control" | "right_control" => {
        crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Ctrl)
      }
      "shift" | "left_shift" | "right_shift" => {
        crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Shift)
      }
      "alt" | "left_option" | "right_option" => {
        crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Alt)
      }
      "super" | "left_command" | "right_command" => {
        crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Super)
      }
      "enter" | "return" | "\n" => {
        crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Enter, '\n')
      }
      "space" | " " => {
        crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Space, ' ')
      }
      "tab" | "\t" => {
        crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Tab, '\t')
      }
      "escape" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Escape),
      "delete" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Delete),
      "backspace" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Backspace),
      "arrow_right" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::ArrowRight),
      "arrow_down" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::ArrowDown),
      "arrow_left" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::ArrowLeft),
      "arrow_up" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::ArrowUp),
      "null" => crate::keyboards::KeyCode::Null,
      string if string == "" => crate::keyboards::KeyCode::Null,
      string => crate::keyboards::KeyCode::Printable(string.chars().next().unwrap()),
    }
  }
}

impl From<String> for KeyCode {
  fn from(name: String) -> KeyCode {
    Self::from(&name as &str)
  }
}

impl TryFrom<u16> for KeyCode {
  type Error = &'static str;

  fn try_from(code: u16) -> Result<Self, Self::Error> {
    match code {
      0 => Ok(KeyCode::from("a")),
      11 => Ok(KeyCode::from("b")),
      8 => Ok(KeyCode::from("c")),
      2 => Ok(KeyCode::from("d")),
      14 => Ok(KeyCode::from("e")),
      3 => Ok(KeyCode::from("f")),
      5 => Ok(KeyCode::from("g")),
      4 => Ok(KeyCode::from("h")),
      34 => Ok(KeyCode::from("i")),
      38 => Ok(KeyCode::from("j")),
      40 => Ok(KeyCode::from("k")),
      37 => Ok(KeyCode::from("l")),
      46 => Ok(KeyCode::from("m")),
      45 => Ok(KeyCode::from("n")),
      31 => Ok(KeyCode::from("o")),
      35 => Ok(KeyCode::from("p")),
      12 => Ok(KeyCode::from("q")),
      15 => Ok(KeyCode::from("r")),
      1 => Ok(KeyCode::from("s")),
      17 => Ok(KeyCode::from("t")),
      32 => Ok(KeyCode::from("u")),
      9 => Ok(KeyCode::from("v")),
      13 => Ok(KeyCode::from("w")),
      7 => Ok(KeyCode::from("x")),
      16 => Ok(KeyCode::from("y")),
      6 => Ok(KeyCode::from("z")),
      29 => Ok(KeyCode::from("0")),
      18 => Ok(KeyCode::from("1")),
      19 => Ok(KeyCode::from("2")),
      20 => Ok(KeyCode::from("3")),
      21 => Ok(KeyCode::from("4")),
      23 => Ok(KeyCode::from("5")),
      22 => Ok(KeyCode::from("6")),
      26 => Ok(KeyCode::from("7")),
      28 => Ok(KeyCode::from("8")),
      25 => Ok(KeyCode::from("9")),
      27 => Ok(KeyCode::from("-")),
      24 => Ok(KeyCode::from("=")),
      33 => Ok(KeyCode::from("[")),
      30 => Ok(KeyCode::from("]")),
      42 => Ok(KeyCode::from("\\")),
      41 => Ok(KeyCode::from(";")),
      50 => Ok(KeyCode::from("`")),
      43 => Ok(KeyCode::from(",")),
      47 => Ok(KeyCode::from(".")),
      44 => Ok(KeyCode::from("/")),
      122 => Ok(KeyCode::from("F1")),
      120 => Ok(KeyCode::from("F2")),
      99 => Ok(KeyCode::from("F3")),
      118 => Ok(KeyCode::from("F4")),
      96 => Ok(KeyCode::from("F5")),
      97 => Ok(KeyCode::from("F6")),
      98 => Ok(KeyCode::from("F7")),
      100 => Ok(KeyCode::from("F8")),
      101 => Ok(KeyCode::from("F9")),
      109 => Ok(KeyCode::from("F10")),
      103 => Ok(KeyCode::from("F11")),
      111 => Ok(KeyCode::from("F12")),
      105 => Ok(KeyCode::from("F13")),
      107 => Ok(KeyCode::from("F14")),
      113 => Ok(KeyCode::from("F15")),
      106 => Ok(KeyCode::from("F16")),
      64 => Ok(KeyCode::from("F17")),
      79 => Ok(KeyCode::from("F18")),
      80 => Ok(KeyCode::from("F19")),
      48 => Ok(KeyCode::from("tab")),
      57 => Ok(KeyCode::from("caps_lock")),
      49 => Ok(KeyCode::from("space")),
      52 => Ok(KeyCode::from("return")),
      36 => Ok(KeyCode::from("return")),
      56 => Ok(KeyCode::from("left_shift")),
      60 => Ok(KeyCode::from("right_shift")),
      58 => Ok(KeyCode::from("left_option")),
      61 => Ok(KeyCode::from("right_option")),
      59 => Ok(KeyCode::from("left_control")),
      62 => Ok(KeyCode::from("right_control")),
      55 => Ok(KeyCode::from("left_command")),
      54 => Ok(KeyCode::from("right_command")),
      51 => Ok(KeyCode::from("delete")),
      53 => Ok(KeyCode::from("escape")),
      123 => Ok(KeyCode::from("arrow_left")),
      124 => Ok(KeyCode::from("arrow_right")),
      126 => Ok(KeyCode::from("arrow_up")),
      125 => Ok(KeyCode::from("arrow_down")),
      63 => Ok(KeyCode::from("fn")),
      116 => Ok(KeyCode::from("home")),
      121 => Ok(KeyCode::from("end")),
      115 => Ok(KeyCode::from("page_up")),
      119 => Ok(KeyCode::from("page_down")),
      71 => Ok(KeyCode::from("clear")),
      81 => Ok(KeyCode::from("=")),
      75 => Ok(KeyCode::from("/")),
      67 => Ok(KeyCode::from("*")),
      78 => Ok(KeyCode::from("–")),
      69 => Ok(KeyCode::from("+")),
      76 => Ok(KeyCode::from("enter")),
      82 => Ok(KeyCode::from("0")),
      83 => Ok(KeyCode::from("1")),
      84 => Ok(KeyCode::from("2")),
      85 => Ok(KeyCode::from("3")),
      86 => Ok(KeyCode::from("4")),
      87 => Ok(KeyCode::from("5")),
      88 => Ok(KeyCode::from("6")),
      89 => Ok(KeyCode::from("7")),
      91 => Ok(KeyCode::from("8")),
      92 => Ok(KeyCode::from("9")),
      _ => Err(""),
    }
  }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
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

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
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

  mod key_combination {
    use super::*;

    #[test]
    fn fulfilled() {
      let combination = combo![KeyCode::from("ctrl"), KeyCode::from("j")];

      assert!(combination.fulfilled(&set![KeyCode::from("ctrl"), KeyCode::from("j")]));
      assert!(!combination.fulfilled(&set![KeyCode::from("a")]));
    }
  }

  mod key_combinations {
    use super::*;

    #[test]

    fn fulfilled() {
      let combination = combos![
        combo![KeyCode::from("a")],
        combo![KeyCode::from("ctrl"), KeyCode::from("j")]
      ];

      assert!(combination.fulfilled(&set![KeyCode::from("a")]));
      assert!(combination.fulfilled(&set![KeyCode::from("ctrl"), KeyCode::from("j")]));
      assert!(!combination.fulfilled(&set![KeyCode::from("b")]));
    }
  }
}

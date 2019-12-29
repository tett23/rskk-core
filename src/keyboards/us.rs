use super::{KeyCode, Keyboard};
use std::collections::HashSet;

#[derive(Clone)]
pub struct US {
  pressing_keys: HashSet<KeyCode>,
  last_character: Option<char>,
}

// impl Clone for HashSet<KeyCode> {
//   fn clone(&self) -> Self {
//     let ret =HashSet::new();
//     self.clone()
//   }
// }
// impl Copy for HashSet<KeyCode> {}

impl US {
  pub fn new() -> Self {
    US {
      pressing_keys: HashSet::new(),
      last_character: None,
    }
  }

  fn convert(key: &KeyCode, is_shift_down: bool) -> Option<char> {
    match (key.printable_key(), is_shift_down) {
      (Some(character), false) => Some(character.clone()),
      (Some(character), true) => Some(match character {
        'a' => 'A',
        'b' => 'B',
        'c' => 'C',
        'd' => 'D',
        'e' => 'E',
        'f' => 'F',
        'g' => 'G',
        'h' => 'H',
        'i' => 'I',
        'j' => 'J',
        'k' => 'K',
        'l' => 'L',
        'm' => 'M',
        'n' => 'N',
        'o' => 'O',
        'p' => 'P',
        'q' => 'Q',
        'r' => 'R',
        's' => 'S',
        't' => 'T',
        'u' => 'U',
        'w' => 'W',
        'x' => 'X',
        'y' => 'Y',
        'z' => 'Z',
        '1' => '!',
        '2' => '@',
        '3' => '#',
        '4' => '$',
        '5' => '%',
        '6' => '^',
        '7' => '&',
        '8' => '*',
        '9' => '(',
        '0' => ')',
        '-' => '_',
        '=' => '+',
        '[' => '{',
        ']' => '}',
        ';' => ':',
        '/' => '?',
        ',' => '<',
        '\\' => '|',
        '`' => '~',
        _ => character.clone(),
      }),
      _ => None,
    }
  }
}

impl Keyboard for US {
  fn pressing_keys(&self) -> &HashSet<KeyCode> {
    &self.pressing_keys
  }

  fn last_character(&self) -> Option<char> {
    self.last_character.clone()
  }

  fn key_down(&mut self, key: &KeyCode) {
    // 絵文字直接入力に対応するため、コードポイントを渡せるようにしたほうがいい？
    // non USなキーボード時に記号入力がおかしくなりそう
    // キーボードの抽象化の層が必要では？
    self.pressing_keys.insert(key.clone());
    self.last_character = US::convert(key, self.is_pressing_shift());
  }

  fn key_up(&mut self, key: &KeyCode) {
    self.pressing_keys.remove(key);
  }
}

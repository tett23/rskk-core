use super::KeyCode;
use super::Keyboard;
use std::collections::HashSet;

pub struct US {
  pressing_keys: HashSet<KeyCode>,
  last_character: Option<char>,
}

impl US {
  pub fn new() -> Self {
    US {
      pressing_keys: HashSet::new(),
      last_character: None,
    }
  }

  fn convert(key: &KeyCode, is_shift_down: bool) -> Option<char> {
    match (key, is_shift_down) {
      (KeyCode::KeyA, true) => Some('A'),
      (KeyCode::KeyA, false) => Some('a'),
      (KeyCode::KeyB, true) => Some('B'),
      (KeyCode::KeyB, false) => Some('b'),
      (KeyCode::KeyC, true) => Some('C'),
      (KeyCode::KeyC, false) => Some('c'),
      (KeyCode::KeyD, true) => Some('D'),
      (KeyCode::KeyD, false) => Some('d'),
      (KeyCode::KeyE, true) => Some('E'),
      (KeyCode::KeyE, false) => Some('e'),
      (KeyCode::KeyF, true) => Some('F'),
      (KeyCode::KeyF, false) => Some('f'),
      (KeyCode::KeyG, true) => Some('G'),
      (KeyCode::KeyG, false) => Some('g'),
      (KeyCode::KeyH, true) => Some('H'),
      (KeyCode::KeyH, false) => Some('h'),
      (KeyCode::KeyI, true) => Some('I'),
      (KeyCode::KeyI, false) => Some('i'),
      (KeyCode::KeyJ, true) => Some('J'),
      (KeyCode::KeyJ, false) => Some('j'),
      (KeyCode::KeyK, true) => Some('K'),
      (KeyCode::KeyK, false) => Some('k'),
      (KeyCode::KeyL, true) => Some('L'),
      (KeyCode::KeyL, false) => Some('l'),
      (KeyCode::KeyM, true) => Some('M'),
      (KeyCode::KeyM, false) => Some('m'),
      (KeyCode::KeyN, true) => Some('N'),
      (KeyCode::KeyN, false) => Some('n'),
      (KeyCode::KeyO, true) => Some('O'),
      (KeyCode::KeyO, false) => Some('o'),
      (KeyCode::KeyP, true) => Some('P'),
      (KeyCode::KeyP, false) => Some('p'),
      (KeyCode::KeyQ, true) => Some('Q'),
      (KeyCode::KeyQ, false) => Some('q'),
      (KeyCode::KeyR, true) => Some('R'),
      (KeyCode::KeyR, false) => Some('r'),
      (KeyCode::KeyS, true) => Some('S'),
      (KeyCode::KeyS, false) => Some('s'),
      (KeyCode::KeyT, true) => Some('T'),
      (KeyCode::KeyT, false) => Some('t'),
      (KeyCode::KeyU, true) => Some('U'),
      (KeyCode::KeyU, false) => Some('u'),
      (KeyCode::KeyW, true) => Some('W'),
      (KeyCode::KeyW, false) => Some('w'),
      (KeyCode::KeyX, true) => Some('X'),
      (KeyCode::KeyX, false) => Some('x'),
      (KeyCode::KeyY, true) => Some('Y'),
      (KeyCode::KeyY, false) => Some('y'),
      (KeyCode::KeyZ, true) => Some('Z'),
      (KeyCode::KeyZ, false) => Some('z'),
      (KeyCode::Key1, true) => Some('!'),
      (KeyCode::Key1, false) => Some('1'),
      (KeyCode::Key2, true) => Some('@'),
      (KeyCode::Key2, false) => Some('2'),
      (KeyCode::Key3, true) => Some('#'),
      (KeyCode::Key3, false) => Some('3'),
      (KeyCode::Key4, true) => Some('$'),
      (KeyCode::Key4, false) => Some('4'),
      (KeyCode::Key5, true) => Some('%'),
      (KeyCode::Key5, false) => Some('5'),
      (KeyCode::Key6, true) => Some('^'),
      (KeyCode::Key6, false) => Some('6'),
      (KeyCode::Key7, true) => Some('&'),
      (KeyCode::Key7, false) => Some('7'),
      (KeyCode::Key8, true) => Some('*'),
      (KeyCode::Key8, false) => Some('8'),
      (KeyCode::Key9, true) => Some('('),
      (KeyCode::Key9, false) => Some('9'),
      (KeyCode::Key0, true) => Some(')'),
      (KeyCode::Key0, false) => Some('0'),
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

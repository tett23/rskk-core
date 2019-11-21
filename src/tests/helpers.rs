#![cfg(test)]

use crate::keyboards::keycodes::KeyCode::*;
use crate::keyboards::{KeyCode, KeyEvents};
use KeyEvents::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum KeyManipulation {
  Up,
  Down,
  Repeat,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum KeyToken {
  Normal(char),
  Meta(KeyCode, KeyManipulation),
}

pub fn str_to_key_code_vector(string: &str) -> Vec<KeyEvents> {
  if string.len() == 0 {
    return vec![];
  }

  let (token, consumed) = head_token(string);
  if token.is_none() {
    return vec![];
  }
  let token = token.unwrap();

  let ret = match token {
    KeyToken::Normal(character) => to_key_events(character),
    KeyToken::Meta(key, manipulation) => Some(to_meta_events(key, manipulation)),
  };
  if ret.is_none() {
    return vec![];
  }
  let mut ret = ret.unwrap();

  ret.append(&mut str_to_key_code_vector(&string[consumed..]));

  ret
}

fn head_token(string: &str) -> (Option<KeyToken>, usize) {
  let (head, tail) = string.split_at(1);
  let head = head.chars().next();
  let head = head.unwrap_or(' ');
  if is_normal_token(head) {
    return (Some(KeyToken::Normal(head)), 1);
  }
  if is_meta_start(head) {
    let end_pos = tail.find(']');
    if end_pos.is_none() {
      return (None, tail.len());
    }
    let end_pos = end_pos.unwrap();

    let token = &tail[..end_pos];
    return (parse_meta_token(token), end_pos + 2);
  }

  (None, 1)
}

fn is_normal_token(character: char) -> bool {
  match character {
    'a'..='z' => true,
    'A'..='Z' => true,
    '0'..='9' => true,
    _ => false,
  }
}

fn is_meta_start(character: char) -> bool {
  character == '['
}

fn parse_meta_token(token: &str) -> Option<KeyToken> {
  let idx = token.find(':');
  if idx.is_none() {
    return None;
  }
  let idx = idx.unwrap();

  let (action, key) = token.split_at(idx);
  let key = &key[1..];
  let manipulation = match action {
    "up" | "u" => Some(KeyManipulation::Up),
    "down" | "d" => Some(KeyManipulation::Down),
    "repeat" | "r" => Some(KeyManipulation::Repeat),
    _ => None,
  };

  let key_code = to_key_code(key);

  Some(KeyToken::Meta(key_code?, manipulation?))
}

fn build_events(key: &KeyCode, with_shift: bool) -> Vec<KeyEvents> {
  if with_shift {
    vec![
      KeyDown(Shift),
      KeyDown(key.clone()),
      KeyUp(key.clone()),
      KeyUp(Shift),
    ]
  } else {
    vec![KeyDown(key.clone()), KeyUp(key.clone())]
  }
}

fn to_key_events(character: char) -> Option<Vec<KeyEvents>> {
  let lowercase = character.to_lowercase().to_string();
  let key = to_key_code(&*lowercase);

  Some(build_events(&key?, character.is_ascii_uppercase()))
}

fn to_key_code(character: &str) -> Option<KeyCode> {
  match character {
    "a" => Some(KeyA),
    "b" => Some(KeyB),
    "c" => Some(KeyC),
    "d" => Some(KeyD),
    "e" => Some(KeyE),
    "f" => Some(KeyF),
    "g" => Some(KeyG),
    "h" => Some(KeyH),
    "i" => Some(KeyI),
    "j" => Some(KeyJ),
    "k" => Some(KeyK),
    "l" => Some(KeyL),
    "m" => Some(KeyM),
    "n" => Some(KeyN),
    "o" => Some(KeyO),
    "p" => Some(KeyP),
    "q" => Some(KeyQ),
    "r" => Some(KeyR),
    "s" => Some(KeyS),
    "t" => Some(KeyT),
    "u" => Some(KeyU),
    "w" => Some(KeyW),
    "x" => Some(KeyX),
    "y" => Some(KeyY),
    "z" => Some(KeyZ),
    "0" => Some(Key0),
    "1" => Some(Key1),
    "2" => Some(Key2),
    "3" => Some(Key3),
    "4" => Some(Key4),
    "5" => Some(Key5),
    "6" => Some(Key6),
    "7" => Some(Key7),
    "8" => Some(Key8),
    "9" => Some(Key9),
    "ctrl" => Some(Ctrl),
    "shift" => Some(Shift),
    "alt" => Some(Alt),
    "super" => Some(Super),
    "slash" => Some(Slash),
    "enter" => Some(Enter),
    _ => None,
  }
}

fn to_meta_events(key: KeyCode, manipulation: KeyManipulation) -> Vec<KeyEvents> {
  match manipulation {
    KeyManipulation::Down => vec![KeyDown(key)],
    KeyManipulation::Up => vec![KeyUp(key)],
    KeyManipulation::Repeat => vec![KeyRepeat(key)],
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let events = str_to_key_code_vector("A[down:shift]b[up:shift]");
    assert_eq!(
      events,
      vec![
        KeyDown(Shift),
        KeyDown(KeyA),
        KeyUp(KeyA),
        KeyUp(Shift),
        KeyDown(Shift),
        KeyDown(KeyB),
        KeyUp(KeyB),
        KeyUp(Shift)
      ]
    );
  }
}

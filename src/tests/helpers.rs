#![cfg(test)]

use crate::keyboards::{KeyCode, KeyEvents, MetaKey};
use crate::transformers::Config;
use crate::{key, set, Dictionary, RSKKConfig};
use std::rc::Rc;
use KeyEvents::*;

pub fn dummy_conf() -> Config {
  Config::new(
    Rc::new(RSKKConfig::default_config()),
    Rc::new(Dictionary::new(set![])),
  )
}

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

  Some(KeyToken::Meta(key_code, manipulation?))
}

fn build_events(key: &KeyCode, with_shift: bool) -> Vec<KeyEvents> {
  if with_shift {
    vec![
      KeyDown(KeyCode::Meta(MetaKey::Shift)),
      KeyDown(key.clone()),
      KeyUp(key.clone()),
      KeyUp(KeyCode::Meta(MetaKey::Shift)),
    ]
  } else {
    vec![KeyDown(key.clone()), KeyUp(key.clone())]
  }
}

fn to_key_events(character: char) -> Option<Vec<KeyEvents>> {
  let lowercase = character.to_lowercase().to_string();
  let key = to_key_code(&*lowercase);

  Some(build_events(&key, character.is_ascii_uppercase()))
}

fn to_key_code(character: &str) -> KeyCode {
  key!(character)
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
        KeyDown(key!("shift")),
        KeyDown(key!("a")),
        KeyUp(key!("a")),
        KeyUp(key!("shift")),
        KeyDown(key!("shift")),
        KeyDown(key!("b")),
        KeyUp(key!("b")),
        KeyUp(key!("shift")),
      ]
    );
  }
}

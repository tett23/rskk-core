#![cfg(test)]

#[macro_use]
pub mod transformer;

use crate::keyboards::{KeyCode, KeyEvents, MetaKey};
use crate::transformers::Config;
use crate::{key, Dictionary, RSKKConfig};
use std::rc::Rc;
use KeyEvents::*;

pub use transformer::*;

pub fn dummy_conf() -> Config {
  Config::new(
    Rc::new(RSKKConfig::default_config()),
    Rc::new(Dictionary::parse(
      "
かんじ/漢字/
みち/未知/道/
ご/語/
    ",
    )),
  )
}

pub fn str_to_key_code_vector(string: &str) -> Vec<KeyEvents> {
  if string.len() == 0 {
    return vec![];
  }

  let (vec, consumed) = head_token(string);
  if vec.is_none() {
    return vec![];
  }
  let mut vec = vec.unwrap();

  vec.append(&mut str_to_key_code_vector(&string[consumed..]));

  vec
}

fn head_token(string: &str) -> (Option<Vec<KeyEvents>>, usize) {
  let (head, tail) = string.split_at(1);
  let head = head.chars().next();
  let head = head.unwrap_or(' ');
  if is_meta_start(head) {
    let end_pos = tail.find(']');
    if end_pos.is_none() {
      return (None, tail.len());
    }
    let end_pos = end_pos.unwrap();

    let token = &tail[..end_pos];
    return (parse_token(token), end_pos + 2);
  }

  return (parse_token(&head.to_string()), 1);
}

fn is_meta_start(character: char) -> bool {
  character == '['
}

fn parse_token(token: &str) -> Option<Vec<KeyEvents>> {
  let idx = token.find(':');
  if idx.is_none() {
    return to_key_events(token);
  }
  let idx = idx.unwrap();

  let (action, key) = token.split_at(idx);
  let key = key!(&key[1..]);
  match action {
    "up" => Some(vec![KeyEvents::KeyUp(key)]),
    "down" => Some(vec![KeyEvents::KeyDown(key)]),
    "repeat" => Some(vec![KeyEvents::KeyRepeat(key)]),
    _ => None,
  }
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

fn to_key_events(character: &str) -> Option<Vec<KeyEvents>> {
  let lowercase = character.to_lowercase().to_string();
  let key = key!(&*lowercase);

  Some(build_events(
    &key,
    character.chars().next().unwrap_or(' ').is_ascii_uppercase(),
  ))
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

    let events = str_to_key_code_vector("[ctrl]");
    assert_eq!(events, vec![KeyDown(key!("ctrl")), KeyUp(key!("ctrl"))]);

    let events = str_to_key_code_vector("[a]");
    assert_eq!(events, vec![KeyDown(key!("a")), KeyUp(key!("a"))]);

    let events = str_to_key_code_vector(" \n");
    assert_eq!(
      events,
      vec![
        KeyDown(key!(" ")),
        KeyUp(key!(" ")),
        KeyDown(key!("\n")),
        KeyUp(key!("\n"))
      ]
    );
  }
}

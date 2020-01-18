use super::direct;
use super::hiragana;
use super::{BufferState, LetterType};
use LetterType::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BufferPair {
  buffer: String,
  state: BufferState,
  letter_type: LetterType,
}

impl BufferPair {
  pub fn new<S: Into<String>>(letter_type: LetterType, buffer: S, state: BufferState) -> Self {
    BufferPair {
      buffer: buffer.into(),
      state,
      letter_type,
    }
  }

  pub fn new_empty(letter_type: LetterType) -> Self {
    Self::new(letter_type, "", BufferState::Continue)
  }

  pub fn push(&self, character: char) -> Option<Vec<BufferPair>> {
    match &self.letter_type {
      &Direct => direct::convert(&self.buffer, character),
      &Hiragana => hiragana::convert(&self.buffer, character),
      &Katakana => unimplemented!(),
      &EnKatakana => unimplemented!(),
      &EmEisu => unimplemented!(),
    }
  }

  pub fn is_stopped(&self) -> bool {
    self.state == BufferState::Stop
  }

  pub fn is_empty(&self) -> bool {
    self.buffer.is_empty()
  }

  pub fn to_string(&self) -> String {
    self.buffer.clone()
  }

  pub fn state(&self) -> BufferState {
    self.state
  }

  pub fn remove_last(&mut self) -> Option<char> {
    let mut vec = self.buffer.chars().collect::<Vec<char>>();
    let character = vec.pop();

    self.buffer = vec
      .into_iter()
      .fold(String::new(), |acc, c| acc + &c.to_string());

    character
  }
}

#[cfg(test)]
mod tests {
  use super::BufferState::*;
  use super::*;

  #[test]
  fn push() {
    let pair = BufferPair::new_empty(Hiragana);
    assert_eq!(
      pair.push('a'),
      Some(vec![BufferPair::new(Hiragana, "あ", Stop)])
    );

    let pair = BufferPair::new_empty(Hiragana);
    assert_eq!(
      pair.push('t'),
      Some(vec![BufferPair::new(Hiragana, "t", Continue)])
    );

    let pair = BufferPair::new(Hiragana, "t", Continue);
    assert_eq!(
      pair.push('a'),
      Some(vec![BufferPair::new(Hiragana, "た", Stop)])
    );

    let pair = BufferPair::new(Hiragana, "t", Continue);
    assert_eq!(
      pair.push('t'),
      Some(vec![
        BufferPair::new(Hiragana, "っ", Stop),
        BufferPair::new(Hiragana, "t", Continue)
      ])
    );
  }

  #[test]
  fn remove_last() {
    let mut pair = BufferPair::new_empty(Hiragana);
    assert_eq!(pair.remove_last(), None);
    assert_eq!(pair.buffer, "");

    let mut pair = BufferPair::new_empty(Hiragana)
      .push('a')
      .unwrap()
      .pop()
      .unwrap();
    assert_eq!(pair.remove_last(), Some('あ'));
    assert_eq!(pair.buffer, "");

    let mut pair = BufferPair::new_empty(Hiragana)
      .push('t')
      .unwrap()
      .pop()
      .unwrap();
    assert_eq!(pair.remove_last(), Some('t'));
    assert_eq!(pair.buffer, "");

    let mut pair = BufferPair::new_empty(Hiragana)
      .push('t')
      .unwrap()
      .pop()
      .unwrap()
      .push('s')
      .unwrap()
      .pop()
      .unwrap();
    assert_eq!(pair.remove_last(), Some('s'));
    assert_eq!(pair.buffer, "t");
  }
}

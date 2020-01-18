use super::hiragana;
use super::{BufferState, LetterType};

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
    hiragana::convert(&self.buffer, character)
  }

  pub fn is_stopped(&self) -> bool {
    self.state == BufferState::Stop
  }

  pub fn to_string(&self) -> String {
    self.buffer.clone()
  }

  pub fn state(&self) -> BufferState {
    self.state
  }
}

#[cfg(test)]
mod tests {
  use super::BufferState::*;
  use super::*;
  use LetterType::*;

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
}

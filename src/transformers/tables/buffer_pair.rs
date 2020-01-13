use super::hiragana;
use super::BufferState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BufferPair(String, BufferState);

impl BufferPair {
  pub fn new<S: Into<String>>(buf: S, state: BufferState) -> Self {
    BufferPair(buf.into(), state)
  }

  pub fn new_empty() -> Self {
    Self::new("", BufferState::Continue)
  }

  pub fn push(&self, character: char) -> Option<Vec<BufferPair>> {
    hiragana::convert(&self.0, character)
  }

  pub fn is_stopped(&self) -> bool {
    self.1 == BufferState::Stop
  }

  pub fn to_string(&self) -> String {
    self.0.clone()
  }

  pub fn state(&self) -> BufferState {
    self.1
  }
}

#[cfg(test)]
mod tests {
  use super::BufferState::*;
  use super::*;

  #[test]
  fn push() {
    let pair = BufferPair::new_empty();
    assert_eq!(pair.push('a'), Some(vec![BufferPair::new("あ", Stop)]));

    let pair = BufferPair::new_empty();
    assert_eq!(pair.push('t'), Some(vec![BufferPair::new("t", Continue)]));

    let pair = BufferPair::new("t", Continue);
    assert_eq!(pair.push('a'), Some(vec![BufferPair::new("た", Stop)]));

    let pair = BufferPair::new("t", Continue);
    assert_eq!(
      pair.push('t'),
      Some(vec![
        BufferPair::new("っ", Stop),
        BufferPair::new("t", Continue)
      ])
    );
  }
}

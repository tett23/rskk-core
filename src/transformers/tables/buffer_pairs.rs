use super::{BufferPair, BufferState, LetterType};

#[derive(Clone, Debug)]
pub struct BufferPairs {
  buffer: Vec<BufferPair>,
  letter_type: LetterType,
}

impl BufferPairs {
  pub fn new(letter_type: LetterType) -> Self {
    BufferPairs {
      buffer: vec![],
      letter_type,
    }
  }

  pub fn push(&mut self, character: char) {
    match &*self.buffer {
      [] => self.push_new_pair(),
      [.., last] if last.is_stopped() => self.push_new_pair(),
      _ => {}
    }

    self.buffer.pop().and_then(|pair| {
      pair
        .push(character)
        .map(|vec| vec.into_iter().for_each(|pair| self.buffer.push(pair)))
    });
  }

  pub fn is_stopped(&self) -> bool {
    self
      .buffer
      .last()
      .map(|item| item.is_stopped())
      .unwrap_or(false)
  }

  pub fn remove_last(&mut self) -> Option<BufferPair> {
    let mut pair = self.buffer.pop()?;
    match pair.state() {
      BufferState::Stop => Some(pair),
      BufferState::Continue => {
        let character = pair.remove_last()?;
        let ret = BufferPair::new(self.letter_type, character.to_string(), BufferState::Stop);
        if !pair.is_empty() {
          self.buffer.push(pair);
        }

        Some(ret)
      }
    }
  }

  pub fn to_string(&self) -> String {
    self
      .buffer
      .iter()
      .fold(String::new(), |acc, buf| acc + &buf.to_string())
  }

  pub fn is_empty(&self) -> bool {
    self.buffer.is_empty()
  }

  pub fn letter_type(&self) -> LetterType {
    self.letter_type
  }

  fn push_new_pair(&mut self) {
    self.buffer.push(BufferPair::new_empty(self.letter_type))
  }
}

impl From<(LetterType, &str)> for BufferPairs {
  fn from((letter_type, buffer): (LetterType, &str)) -> Self {
    buffer
      .chars()
      .collect::<Vec<char>>()
      .into_iter()
      .fold(Self::new(letter_type), |mut acc, c| {
        acc.push(c);
        acc
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use LetterType::*;

  #[test]
  fn is_stopped() {
    assert_eq!(BufferPairs::from((Hiragana, "")).is_stopped(), false);
    assert_eq!(BufferPairs::from((Hiragana, "a")).is_stopped(), true);
    assert_eq!(BufferPairs::from((Hiragana, "k")).is_stopped(), false);
    assert_eq!(BufferPairs::from((Hiragana, "ka")).is_stopped(), true);
    assert_eq!(BufferPairs::from((Hiragana, "tt")).is_stopped(), false);
    assert_eq!(BufferPairs::from((Hiragana, "tte")).is_stopped(), true);
    assert_eq!(BufferPairs::from((Hiragana, "atte")).is_stopped(), true);
  }

  #[test]
  fn from() {
    assert_eq!(&BufferPairs::from((Hiragana, "")).to_string(), "");
    assert_eq!(&BufferPairs::from((Hiragana, "a")).to_string(), "あ");
    assert_eq!(&BufferPairs::from((Hiragana, "k")).to_string(), "k");
    assert_eq!(&BufferPairs::from((Hiragana, "ka")).to_string(), "か");
    assert_eq!(&BufferPairs::from((Hiragana, "tte")).to_string(), "って");
    assert_eq!(
      &BufferPairs::from((Hiragana, "hogehoge")).to_string(),
      "ほげほげ"
    );
  }

  #[test]
  fn remove_last() {
    let mut pairs = BufferPairs::from((Hiragana, ""));
    assert_eq!(pairs.remove_last(), None);
    assert_eq!(pairs.buffer, vec![]);

    let mut pairs = BufferPairs::from((Hiragana, "a"));
    assert_eq!(
      pairs.remove_last(),
      Some(BufferPair::new(pairs.letter_type, "あ", BufferState::Stop))
    );
    assert_eq!(pairs.buffer, vec![]);

    let mut pairs = BufferPairs::from((Hiragana, "ts"));
    assert_eq!(
      pairs.remove_last(),
      Some(BufferPair::new(pairs.letter_type, "s", BufferState::Stop))
    );
    assert_eq!(
      pairs.buffer,
      vec![BufferPair::new(
        pairs.letter_type,
        "t",
        BufferState::Continue
      )]
    );

    let mut pairs = BufferPairs::from((Hiragana, "tt"));
    assert_eq!(
      pairs.remove_last(),
      Some(BufferPair::new(pairs.letter_type, "t", BufferState::Stop))
    );
    assert_eq!(
      pairs.buffer,
      vec![BufferPair::new(pairs.letter_type, "っ", BufferState::Stop)]
    );
  }
}

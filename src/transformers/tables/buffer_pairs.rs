use super::{BufferPair, LetterType};

#[derive(Clone, Debug)]
pub struct BufferPairs {
  buffer: Vec<BufferPair>,
  letter_type: LetterType,
}

impl BufferPairs {
  pub fn new(letter_type: LetterType) -> Self {
    BufferPairs {
      buffer: vec![BufferPair::new_empty(letter_type)],
      letter_type,
    }
  }

  pub fn push(&mut self, character: char) {
    if let Some(pair) = self.buffer.last() {
      if pair.is_stopped() {
        self.buffer.push(BufferPair::new_empty(self.letter_type))
      }
    };
    let pair = self
      .buffer
      .pop()
      .unwrap_or(BufferPair::new_empty(self.letter_type));

    pair.push(character).and_then(|vec| {
      vec.into_iter().for_each(|pair| self.buffer.push(pair));
      Some(())
    });
  }

  pub fn is_stopped(&self) -> bool {
    self
      .buffer
      .last()
      .map(|item| item.is_stopped())
      .unwrap_or(false)
  }

  pub fn pop(&mut self) -> Option<BufferPair> {
    self.buffer.pop()
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
}

impl From<(LetterType, &str)> for BufferPairs {
  fn from((letter_type, buffer): (LetterType, &str)) -> Self {
    buffer.chars().collect::<Vec<char>>().into_iter().fold(
      BufferPairs::new(letter_type),
      |mut acc, c| {
        acc.push(c);
        acc
      },
    )
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
}

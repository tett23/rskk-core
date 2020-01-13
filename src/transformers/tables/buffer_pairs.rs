use super::BufferPair;

#[derive(Clone, Debug)]
pub struct BufferPairs(Vec<BufferPair>);

impl BufferPairs {
  pub fn new() -> Self {
    BufferPairs(vec![BufferPair::new_empty()])
  }

  pub fn push(&mut self, character: char) {
    if let Some(pair) = self.0.last() {
      if pair.is_stopped() {
        self.0.push(BufferPair::new_empty())
      }
    };
    let pair = self.0.pop().unwrap_or(BufferPair::new_empty());

    pair.push(character).and_then(|vec| {
      vec.into_iter().for_each(|pair| self.0.push(pair));
      Some(())
    });
  }

  pub fn is_stopped(&self) -> bool {
    self.0.last().map(|item| item.is_stopped()).unwrap_or(false)
  }

  pub fn pop(&mut self) -> Option<BufferPair> {
    self.0.pop()
  }

  pub fn to_string(&self) -> String {
    self
      .0
      .iter()
      .fold(String::new(), |acc, buf| acc + &buf.to_string())
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl From<&str> for BufferPairs {
  fn from(buffer: &str) -> Self {
    buffer
      .chars()
      .collect::<Vec<char>>()
      .into_iter()
      .fold(BufferPairs::new(), |mut acc, c| {
        acc.push(c);
        acc
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_stopped() {
    assert_eq!(BufferPairs::from("").is_stopped(), false);
    assert_eq!(BufferPairs::from("a").is_stopped(), true);
    assert_eq!(BufferPairs::from("k").is_stopped(), false);
    assert_eq!(BufferPairs::from("ka").is_stopped(), true);
    assert_eq!(BufferPairs::from("tt").is_stopped(), false);
    assert_eq!(BufferPairs::from("tte").is_stopped(), true);
    assert_eq!(BufferPairs::from("atte").is_stopped(), true);
  }

  #[test]
  fn from() {
    assert_eq!(&BufferPairs::from("").to_string(), "");
    assert_eq!(&BufferPairs::from("a").to_string(), "あ");
    assert_eq!(&BufferPairs::from("k").to_string(), "k");
    assert_eq!(&BufferPairs::from("ka").to_string(), "か");
    assert_eq!(&BufferPairs::from("tte").to_string(), "って");
    assert_eq!(&BufferPairs::from("hogehoge").to_string(), "ほげほげ");
  }

  #[test]
  fn pop() {}
}

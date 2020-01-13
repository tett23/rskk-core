use super::tables::BufferPairs;
use super::Displayable;

#[derive(Clone, Debug)]
pub struct YomiPair(BufferPairs, Option<BufferPairs>);

impl YomiPair {
  pub fn new() -> Self {
    YomiPair(BufferPairs::new(), None)
  }

  pub fn push(&mut self, character: char) {
    if let Some(okuri) = &mut self.1 {
      okuri.push(character);
      self.1 = Some(okuri.clone());
    } else {
      self.0.push(character);
    }
  }

  pub fn pop(&mut self) {
    match (&mut self.0, &mut self.1) {
      (_, Some(_)) => {
        self.1 = None;
      }
      (yomi, None) => {
        yomi.pop();
      }
    }
  }

  pub fn start_okuri(&mut self) {
    self.1 = Some(BufferPairs::new())
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn is_stopped(&self) -> bool {
    match &self.1 {
      None => false,
      Some(yomi) => yomi.is_stopped(),
    }
  }

  fn yomi_string(&self) -> String {
    self.0.to_string()
  }

  fn okuri_string(&self) -> Option<String> {
    Some(self.1.clone()?.to_string())
  }
}

#[derive(Clone, Debug)]
pub struct Word {
  pair: YomiPair,
  okuri: Option<char>,
}

impl Word {
  pub fn new() -> Self {
    Word {
      pair: YomiPair::new(),
      okuri: None,
    }
  }

  pub fn push(&mut self, character: char) {
    let character = match self.is_empty() {
      true => character.to_lowercase().next().unwrap(),
      false => character,
    };
    let lowercase = character.to_lowercase().next().unwrap();
    if character.is_ascii_uppercase() {
      self.pair.start_okuri();
      self.okuri = Some(lowercase);
    }

    self.pair.push(lowercase);
  }

  pub fn pop(&mut self) {
    self.pair.pop();
    if self.pair.1.is_none() {
      self.okuri = None;
    }
  }

  pub fn remove_okuri(&mut self) {
    self.pair.1 = None;
    self.okuri = None;
  }

  pub fn okuri_string(&self) -> Option<String> {
    self.pair.okuri_string()
  }

  pub fn has_okuri(&self) -> bool {
    self.okuri.is_some()
  }

  pub fn is_empty(&self) -> bool {
    self.pair.is_empty()
  }

  pub fn is_stopped(&self) -> bool {
    self.pair.is_stopped()
  }

  pub fn to_dic_read(&self) -> Option<String> {
    let yomi = self.pair.yomi_string();
    if yomi.is_empty() {
      return None;
    }

    Some(match self.okuri {
      Some(character) => self.pair.yomi_string() + &character.to_string(),
      None => self.pair.yomi_string(),
    })
  }

  fn to_string_pair(&self) -> (String, Option<String>) {
    (self.pair.yomi_string(), self.pair.okuri_string())
  }
}

impl Displayable for Word {
  fn buffer_content(&self) -> String {
    match self.to_string_pair() {
      (yomi, Some(okuri)) => yomi + &okuri,
      (yomi, None) => yomi,
    }
  }

  fn display_string(&self) -> String {
    match self.to_string_pair() {
      (yomi, Some(okuri)) => yomi + "*" + &okuri,
      (yomi, None) => yomi,
    }
  }
}

impl From<&str> for Word {
  fn from(item: &str) -> Self {
    item.chars().fold(Word::new(), |mut acc, c| {
      acc.push(c);
      acc
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn to_dic_read() {
    assert_eq!(Word::from("").to_dic_read(), None);
    assert_eq!(&Word::from("a").to_dic_read().unwrap(), "あ");
    assert_eq!(&Word::from("aTte").to_dic_read().unwrap(), "あt");
  }

  #[test]
  fn to_string_pair() {
    assert_eq!(Word::from("").to_string_pair(), ("".to_owned(), None));
    assert_eq!(Word::from("a").to_string_pair(), ("あ".to_owned(), None));
    assert_eq!(
      Word::from("aTte").to_string_pair(),
      ("あ".to_owned(), Some("って".to_owned()))
    );
    assert_eq!(
      Word::from("okuR").to_string_pair(),
      ("おく".to_owned(), Some("r".to_owned()))
    );
  }

  #[test]
  fn display_string() {
    assert_eq!(&Word::from("").display_string(), "");
    assert_eq!(&Word::from("hiragana").display_string(), "ひらがな");
    assert_eq!(&Word::from("aTte").display_string(), "あ*って");
    assert_eq!(&Word::from("okuR").display_string(), "おく*r");
  }
}

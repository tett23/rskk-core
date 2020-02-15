use super::tables::BufferPairs;
use super::Displayable;
use super::LetterType;

#[derive(Clone, Debug)]
pub struct YomiPair(BufferPairs, Option<BufferPairs>);

impl YomiPair {
  pub fn new(letter_type: LetterType) -> Self {
    YomiPair(BufferPairs::new(letter_type), None)
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
        yomi.remove_last();
      }
    }
  }

  pub fn start_okuri(&mut self) {
    self.1 = Some(BufferPairs::new(self.0.letter_type()))
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

impl From<(LetterType, &str)> for YomiPair {
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

#[derive(Clone, Debug)]
pub struct Word {
  pair: YomiPair,
  dic_read: BufferPairs,
  okuri: Option<char>,
}

impl Word {
  pub fn new(letter_type: LetterType) -> Self {
    Self {
      pair: YomiPair::new(letter_type),
      dic_read: BufferPairs::new(LetterType::Hiragana),
      okuri: None,
    }
  }

  pub fn new_abbr<S: Into<String>>(buf: S) -> Self {
    let buf = &buf.into() as &str;
    Self {
      pair: YomiPair::from((LetterType::Direct, buf)),
      dic_read: BufferPairs::from((LetterType::Direct, buf)),
      okuri: None,
    }
  }

  pub fn push(&mut self, character: char) {
    self.try_okuri_start(character);

    let character = character.to_lowercase().next().unwrap();
    if self.okuri.is_none() {
      self.dic_read.push(character);
    }

    self.pair.push(character);
  }

  fn try_okuri_start(&mut self, character: char) {
    if self.is_okuri_start(character) {
      self.pair.start_okuri();
      self.okuri = Some(character.to_lowercase().next().unwrap());
    }
  }

  fn is_okuri_start(&self, character: char) -> bool {
    self.pair.0.is_stopped() && self.okuri.is_none() && character.is_ascii_uppercase()
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
    if self.dic_read.is_empty() {
      return None;
    }

    let read =
      self.dic_read.to_string() + &self.okuri.map(|c| c.to_string()).unwrap_or("".to_owned());

    Some(read)
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

impl From<(LetterType, &str)> for Word {
  fn from((letter_type, item): (LetterType, &str)) -> Self {
    item.chars().fold(Word::new(letter_type), |mut acc, c| {
      acc.push(c);
      acc
    })
  }
}

#[cfg(test)]
mod tests {
  use super::LetterType::*;
  use super::*;

  #[test]
  fn to_dic_read() {
    assert_eq!(Word::from((Hiragana, "")).to_dic_read(), None);
    assert_eq!(&Word::from((Hiragana, "a")).to_dic_read().unwrap(), "あ");
    assert_eq!(
      &Word::from((Hiragana, "aTte")).to_dic_read().unwrap(),
      "あt"
    );

    assert_eq!(Word::from((Katakana, "")).to_dic_read(), None);
    assert_eq!(&Word::from((Katakana, "a")).to_dic_read().unwrap(), "あ");
    assert_eq!(
      &Word::from((Katakana, "aTte")).to_dic_read().unwrap(),
      "あt"
    );
  }

  #[test]
  fn to_string_pair() {
    assert_eq!(
      Word::from((Hiragana, "")).to_string_pair(),
      ("".to_owned(), None)
    );
    assert_eq!(
      Word::from((Hiragana, "a")).to_string_pair(),
      ("あ".to_owned(), None)
    );
    assert_eq!(
      Word::from((Hiragana, "aTte")).to_string_pair(),
      ("あ".to_owned(), Some("って".to_owned()))
    );
    assert_eq!(
      Word::from((Hiragana, "okuR")).to_string_pair(),
      ("おく".to_owned(), Some("r".to_owned()))
    );

    assert_eq!(
      Word::from((Katakana, "")).to_string_pair(),
      ("".to_owned(), None)
    );
    assert_eq!(
      Word::from((Katakana, "a")).to_string_pair(),
      ("ア".to_owned(), None)
    );
    assert_eq!(
      Word::from((Katakana, "aTte")).to_string_pair(),
      ("ア".to_owned(), Some("ッテ".to_owned()))
    );
    assert_eq!(
      Word::from((Katakana, "okuR")).to_string_pair(),
      ("オク".to_owned(), Some("r".to_owned()))
    );
  }

  #[test]
  fn display_string() {
    assert_eq!(&Word::from((Hiragana, "")).display_string(), "");
    assert_eq!(
      &Word::from((Hiragana, "hiragana")).display_string(),
      "ひらがな"
    );
    assert_eq!(&Word::from((Hiragana, "aTte")).display_string(), "あ*って");
    assert_eq!(&Word::from((Hiragana, "okuR")).display_string(), "おく*r");

    assert_eq!(&Word::from((Katakana, "")).display_string(), "");
    assert_eq!(
      &Word::from((Katakana, "hiragana")).display_string(),
      "ヒラガナ"
    );
    assert_eq!(&Word::from((Katakana, "aTte")).display_string(), "ア*ッテ");
    assert_eq!(&Word::from((Katakana, "okuR")).display_string(), "オク*r");
  }
}

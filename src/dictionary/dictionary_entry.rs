use super::Candidate;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DictionaryEntry {
  pub read: String,
  pub candidates: Vec<Candidate>,
}

impl Hash for DictionaryEntry {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.read.hash(state);
    state.finish();
  }
}

impl DictionaryEntry {
  pub fn new(read: String, candidates: Vec<Candidate>) -> Self {
    DictionaryEntry { read, candidates }
  }

  pub fn parse(string: &str) -> Option<Self> {
    let trimmed = string.trim();
    if trimmed.starts_with(";;") {
      return None;
    }

    let items: Vec<_> = trimmed.split_terminator('/').collect();
    if items.len() < 2 {
      return None;
    }
    let mut items = items.iter().filter(|&item| item.len() != 0);

    let read = items.next()?.trim().to_string();
    let mut transforms = Vec::new();
    items.for_each(|&item| {
      if let Some(item) = Candidate::parse(item) {
        transforms.push(item);
      }
    });
    if transforms.len() == 0 {
      return None;
    }

    Some(DictionaryEntry::new(read, transforms))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    let item = DictionaryEntry::parse("a/b;c/d/");
    assert_eq!(
      item,
      Some(DictionaryEntry::new(
        "a".to_string(),
        vec![
          Candidate::new("b".to_string(), Some("c".to_string()),),
          Candidate::new("d".to_string(), None,)
        ],
      ))
    );

    let item = DictionaryEntry::parse("a/b");
    assert_eq!(
      item,
      Some(DictionaryEntry::new(
        "a".to_string(),
        vec![Candidate::new("b".to_string(), None,)],
      ))
    );

    let item = DictionaryEntry::parse(" a / b /");
    assert_eq!(
      item,
      Some(DictionaryEntry::new(
        "a".to_string(),
        vec![Candidate::new("b".to_string(), None,)],
      ))
    );

    let item = DictionaryEntry::parse("a//b//");
    assert_eq!(
      item,
      Some(DictionaryEntry::new(
        "a".to_string(),
        vec![Candidate::new("b".to_string(), None,)],
      ))
    );

    let item = DictionaryEntry::parse("a/");
    assert_eq!(item, None);

    let item = DictionaryEntry::parse(";;");
    assert_eq!(item, None);

    let item = DictionaryEntry::parse(" ;;");
    assert_eq!(item, None);

    let item = DictionaryEntry::parse("");
    assert_eq!(item, None);

    let item = DictionaryEntry::parse(" ");
    assert_eq!(item, None);
  }
}

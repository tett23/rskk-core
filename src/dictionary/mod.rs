mod dictionary_entry;
mod transform_entry;

use dictionary_entry::DictionaryEntry;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Dictionary {
  entries: HashSet<DictionaryEntry>,
}

impl Dictionary {
  pub fn new(set: HashSet<DictionaryEntry>) -> Self {
    return Dictionary { entries: set };
  }

  pub fn insert(&mut self, entry: DictionaryEntry) {
    self.entries.insert(entry);
  }

  pub fn parse(string: &str) -> Self {
    let mut ret = Dictionary::new(HashSet::new());
    for item in string.lines() {
      if let Some(item) = DictionaryEntry::parse(item) {
        ret.insert(item);
      }
    }

    ret
  }
}

#[cfg(test)]
mod tests {
  use super::transform_entry::TransformEntry;
  use super::*;
  use crate::set;

  #[test]
  fn parse() {
    let item = Dictionary::parse("a/b;c/d/");
    assert_eq!(
      item,
      Dictionary::new(set![DictionaryEntry::new(
        "a".to_string(),
        set![
          TransformEntry::new("b".to_string(), Some("c".to_string())),
          TransformEntry::new("d".to_string(), None)
        ],
      )]),
    );

    let item = Dictionary::parse("a/b/\nc/d/");
    assert_eq!(
      item,
      Dictionary::new(set![
        DictionaryEntry::new(
          "a".to_string(),
          set![TransformEntry::new("b".to_string(), None)],
        ),
        DictionaryEntry::new(
          "c".to_string(),
          set![TransformEntry::new("d".to_string(), None)],
        )
      ]),
    );

    let item = Dictionary::parse("a/b/\r\nc/d/");
    assert_eq!(
      item,
      Dictionary::new(set![
        DictionaryEntry::new(
          "a".to_string(),
          set![TransformEntry::new("b".to_string(), None)],
        ),
        DictionaryEntry::new(
          "c".to_string(),
          set![TransformEntry::new("d".to_string(), None)],
        )
      ]),
    );

    let item = Dictionary::parse("a/");
    assert_eq!(item.entries, set![]);

    let item = Dictionary::parse(";;");
    assert_eq!(item.entries, set![]);

    let item = Dictionary::parse("");
    assert_eq!(item.entries, set![]);

    let item = Dictionary::parse(" ");
    assert_eq!(item.entries, set![]);
  }
}

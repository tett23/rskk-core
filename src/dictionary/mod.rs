mod candidate;
mod dictionary_entry;

use std::collections::HashSet;

pub use candidate::Candidate;
pub use dictionary_entry::DictionaryEntry;

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

  pub fn transform(&self, word: &str) -> Option<&DictionaryEntry> {
    // TODO: wordがカタカナの場合があるので正規化する
    // abbrの場合もある
    match self.entries.iter().find(|&item| item.read == word) {
      Some(v) => Some(&v),
      None => None,
    }
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
  use super::*;
  use crate::set;

  #[test]
  fn transform() {
    let kanji = DictionaryEntry::new("かんじ", vec![Candidate::new("漢字", None)]);
    let kanji2 = kanji.clone();
    let okuri = DictionaryEntry::new("おくr", vec![Candidate::new("送", None)]);
    let okuri2 = okuri.clone();
    let dic = Dictionary::new(set![kanji2, okuri2]);

    let entry = dic.transform("かんじ");
    assert_eq!(entry, Some(&kanji));

    let entry = dic.transform("おくr");
    assert_eq!(entry, Some(&okuri));

    let entry = dic.transform("みとうろく");
    assert_eq!(entry, None);
  }

  #[test]
  fn parse() {
    let item = Dictionary::parse("a/b;c/d/");
    assert_eq!(
      item,
      Dictionary::new(set![DictionaryEntry::new(
        "a",
        vec![Candidate::new("b", Some("c")), Candidate::new("d", None)],
      )]),
    );

    let item = Dictionary::parse("a/b/\nc/d/");
    assert_eq!(
      item,
      Dictionary::new(set![
        DictionaryEntry::new("a", vec![Candidate::new("b", None)],),
        DictionaryEntry::new("c", vec![Candidate::new("d", None)],)
      ]),
    );

    let item = Dictionary::parse("a/b/\r\nc/d/");
    assert_eq!(
      item,
      Dictionary::new(set![
        DictionaryEntry::new("a", vec![Candidate::new("b", None)],),
        DictionaryEntry::new("c", vec![Candidate::new("d", None)],)
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

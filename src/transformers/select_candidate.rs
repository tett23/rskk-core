use super::tables::hiragana_convert;
use super::StoppedTransformer;
use super::{
  AsTransformerTrait, BufferState, Config, Displayable, KeyCode, Stackable, Transformable,
  TransformerTypes, WithConfig,
};
use crate::dictionary::{Candidate, DictionaryEntry};
use crate::transformers::tables::convert_from_str;

#[derive(Clone, Debug)]
pub struct SelectCandidateTransformer {
  config: Config,
  buffer: String,
  buffer_state: BufferState,
  dictionary_entry: DictionaryEntry,
  candidates: Candidates,
  okuri: Option<char>,
}

impl SelectCandidateTransformer {
  pub fn new(config: Config, dictionary_entry: &DictionaryEntry, okuri: Option<char>) -> Self {
    SelectCandidateTransformer {
      config,
      buffer: "".to_string(),
      buffer_state: BufferState::Continue,
      dictionary_entry: dictionary_entry.clone(),
      candidates: Candidates::new(&dictionary_entry.candidates),
      okuri,
    }
  }

  fn try_transition_to_stopped(&self) -> Option<Box<dyn Transformable>> {
    self
      .candidates
      .current()
      .map(|candidate| -> Box<dyn Transformable> {
        box StoppedTransformer::completed(
          self.config(),
          Self::append_okuri(&self.dictionary_entry.read, &candidate.entry, self.okuri),
        )
      })
  }

  fn append_okuri(stem: &str, yomi: &str, okuri: Option<char>) -> String {
    stem
      .rfind(|c: char| c.is_ascii_lowercase())
      .map(|idx| stem.split_at(idx))
      .and_then(|pair| match pair {
        (_, "") => None,
        (_, c) => Some(c),
      })
      .and_then(|consonant| Some((consonant, okuri?)))
      .map(|(consonant, vowel)| format!("{}{}", consonant, vowel))
      .and_then(|al| convert_from_str(&al))
      .map(|a| yomi.to_owned() + &a)
      .unwrap_or(yomi.to_owned())
  }
}

impl WithConfig for SelectCandidateTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for SelectCandidateTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::SelectCandidate
  }

  fn push_character(&self, _: char) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    Some(vec![])
  }

  fn push_enter(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.try_transition_to_stopped().map(|tf| vec![tf])
  }

  fn push_space(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let mut new_state = self.clone();
    Some(match new_state.candidates.next() {
      Some(_) => vec![box new_state],
      None => {
        // TODO: 単語登録に遷移
        unimplemented!()
      }
    })
  }

  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    let mut new_state = self.clone();
    Some(match new_state.candidates.prev() {
      Some(_) => vec![box new_state],
      None => vec![],
    })
  }

  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    self.push_delete()
  }

  fn push_any_character(&self, key_code: &KeyCode) -> Option<Vec<Box<dyn Transformable>>> {
    match key_code.is_printable() {
      true => self.try_transition_to_stopped().map(|tf| vec![tf]),
      false => None,
    }
  }
}

impl Displayable for SelectCandidateTransformer {
  fn buffer_content(&self) -> String {
    match (self.candidates.current(), &self.okuri) {
      (Some(candidate), Some(okuri)) => {
        let character = self.dictionary_entry.read.clone().pop();
        if character.is_none() {
          return candidate.entry.clone();
        }
        let character = character.unwrap();

        let okuri = hiragana_convert(&character.to_string(), okuri.clone());
        if okuri.is_none() {
          return candidate.entry.clone();
        }
        let okuri = okuri
          .unwrap()
          .iter()
          .fold("".to_string(), |acc, (s, _)| acc + &s);

        candidate.entry.clone() + &okuri
      }
      (Some(candidate), None) => candidate.entry.clone(),
      (None, _) => "".to_string(),
    }
  }

  fn display_string(&self) -> String {
    "▼".to_string() + &self.buffer_content()
  }
}

impl AsTransformerTrait for SelectCandidateTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    box self.clone()
  }
}

impl Stackable for SelectCandidateTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!()
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    unreachable!()
  }

  fn replace_last_element(&self, _: Vec<Box<dyn Transformable>>) -> Vec<Box<dyn Transformable>> {
    vec![box self.clone()]
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    vec![]
  }
}

#[derive(Clone, Debug)]
struct Candidates {
  candidates: Vec<Candidate>,
  pos: usize,
}

impl Candidates {
  pub fn new(candidates: &Vec<Candidate>) -> Self {
    let mut items = Vec::new();
    for item in candidates.iter() {
      items.push(item.clone());
    }

    Candidates {
      candidates: items,
      pos: 0,
    }
  }

  pub fn next(&mut self) -> Option<&Candidate> {
    if self.pos >= self.candidates.len() {
      return None;
    }

    self.pos += 1;
    self.candidates.get(self.pos)
  }

  pub fn prev(&mut self) -> Option<&Candidate> {
    if self.pos <= 0 {
      return None;
    }

    self.pos -= 1;
    self.candidates.get(self.pos)
  }

  pub fn current(&self) -> Option<&Candidate> {
    self.candidates.get(self.pos)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::transformers::StoppedReason::*;
  use crate::{key, set, Dictionary, RSKKConfig};
  use std::rc::Rc;
  use TransformerTypes::*;

  #[test]
  fn append_okuri() {
    let item = SelectCandidateTransformer::append_okuri("かんじ", "漢字", None);
    assert_eq!(item, "漢字");

    let item = SelectCandidateTransformer::append_okuri("かんじ", "漢字", Some('a'));
    assert_eq!(item, "漢字");

    let item = SelectCandidateTransformer::append_okuri("おくr", "送", Some('i'));
    assert_eq!(item, "送り");
  }

  #[test]
  fn space() {
    let config = Config::new(
      Rc::new(RSKKConfig::default_config()),
      Rc::new(Dictionary::new(set![])),
    );
    let candidate1 = Candidate::new("a", None);
    let candidate2 = Candidate::new("b", None);
    let vec = vec![candidate1.clone(), candidate2.clone()];
    let dictionary_entry = DictionaryEntry::new("test", vec);
    let select_candidate = SelectCandidateTransformer::new(config.clone(), &dictionary_entry, None);

    assert_eq!(select_candidate.buffer_content(), "a");
    assert_eq!(
      select_candidate
        .push_meta_key(&key!("space"))
        .unwrap()
        .iter()
        .fold("".to_owned(), |acc, item| acc + &item.buffer_content()),
      "b"
    );
    // TODO: 単語登録のテストCandidate
  }

  #[test]
  fn enter() {
    let config = Config::new(
      Rc::new(RSKKConfig::default_config()),
      Rc::new(Dictionary::new(set![])),
    );
    let candidate1 = Candidate::new("a", None);
    let vec = vec![candidate1.clone()];
    let dictionary_entry = DictionaryEntry::new("test", vec);
    let select_candidate = SelectCandidateTransformer::new(config.clone(), &dictionary_entry, None);

    let stopped = select_candidate.push_meta_key(&key!("enter")).unwrap();
    let stopped = stopped.first().unwrap();
    assert_eq!(stopped.transformer_type(), Stopped(Compleated));
    assert_eq!(stopped.buffer_content(), "a");
  }

  #[ignore]
  fn delete() {
    let config = Config::new(
      Rc::new(RSKKConfig::default_config()),
      Rc::new(Dictionary::new(set![])),
    );
    let candidate1 = Candidate::new("a", None);
    let candidate2 = Candidate::new("b", None);
    let vec = vec![candidate1.clone(), candidate2.clone()];
    let dictionary_entry = DictionaryEntry::new("test", vec);
    let select_candidate = SelectCandidateTransformer::new(config.clone(), &dictionary_entry, None);

    let select_candidate = select_candidate.push_meta_key(&key!("space")).unwrap();
    let select_candidate = select_candidate.first().unwrap();
    let select_candidate = select_candidate.push_meta_key(&key!("delete")).unwrap();
    let select_candidate = select_candidate.first().unwrap();
    assert_eq!(select_candidate.buffer_content(), "a");

    let canceled = select_candidate.push_meta_key(&key!("delete")).unwrap();
    let canceled = canceled.first().unwrap();
    assert_eq!(canceled.transformer_type(), Stopped(Canceled));
  }

  mod candidates {
    use super::*;

    #[test]
    fn prev() {
      let candidate1 = Candidate::new("a", None);
      let candidate2 = Candidate::new("b", None);
      let vec = vec![candidate1.clone(), candidate2.clone()];
      let mut candidates = Candidates::new(&vec);

      assert_eq!(candidates.current(), Some(&candidate1));
      assert_eq!(candidates.next(), Some(&candidate2));
      assert_eq!(candidates.current(), Some(&candidate2));
      assert_eq!(candidates.next(), None);
      assert_eq!(candidates.next(), None);
      assert_eq!(candidates.current(), None);
      assert_eq!(candidates.prev(), Some(&candidate2));
      assert_eq!(candidates.current(), Some(&candidate2));
      assert_eq!(candidates.prev(), Some(&candidate1));
      assert_eq!(candidates.current(), Some(&candidate1));
      assert_eq!(candidates.prev(), None);
      assert_eq!(candidates.prev(), None);
    }
  }
}

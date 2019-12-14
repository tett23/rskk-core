use super::tables::hiragana_convert;
use super::StoppedTransformer;
use super::{
  AsTransformerTrait, BufferState, Config, Displayable, Stackable, Transformable, TransformerTypes,
  WithConfig,
};
use crate::dictionary::{Candidate, DictionaryEntry};
use crate::keyboards::KeyCode;

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

  fn push_character(&self, _: char) -> Box<dyn Transformable> {
    self.as_trait()
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    Box::new(StoppedTransformer::canceled(self.config()))
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    Box::new(StoppedTransformer::completed(
      self.config(),
      self.buffer_content(),
    ))
  }

  fn push_space(&self) -> Box<dyn Transformable> {
    let mut new_state = self.clone();
    match new_state.candidates.next() {
      Some(_) => Box::new(new_state),
      None => {
        // TODO: 単語登録に遷移
        unimplemented!()
      }
    }
  }

  fn push_delete(&self) -> Box<dyn Transformable> {
    let mut new_state = self.clone();
    match new_state.candidates.prev() {
      Some(_) => Box::new(new_state),
      None => Box::new(StoppedTransformer::canceled(self.config())),
    }
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    self.push_delete()
  }

  fn push_any_character(&self, key_code: &KeyCode) -> Box<dyn Transformable> {
    match key_code.is_printable() {
      true => match self.candidates.current() {
        Some(candidate) => Box::new(StoppedTransformer::completed(
          self.config(),
          candidate.entry.clone(),
        )),
        None => Box::new(StoppedTransformer::canceled(self.config())),
      },
      false => self.as_trait(),
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
        let (okuri, _) = okuri.unwrap();

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
    Box::new(self.clone())
  }
}

impl Stackable for SelectCandidateTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    unreachable!()
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    (self.push_delete(), Some(box self.clone()))
  }

  fn replace_last_element(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    box self.clone()
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
        .buffer_content(),
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

    let stopped = select_candidate.push_meta_key(&key!("enter"));
    assert_eq!(stopped.transformer_type(), Stopped(Compleated));
    assert_eq!(stopped.buffer_content(), "a");
  }

  #[test]
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

    let select_candidate = select_candidate.push_meta_key(&key!("space"));
    let select_candidate = select_candidate.push_meta_key(&key!("delete"));
    assert_eq!(select_candidate.buffer_content(), "a");

    let canceled = select_candidate.push_meta_key(&key!("delete"));
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

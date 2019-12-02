use super::super::{
  AsTransformerTrait, BufferState, Config, Displayable, Transformable, TransformerState,
  TransformerTypes, WithConfig,
};
use super::{Canceled, Stopped};
use crate::dictionary::{Candidate, DictionaryEntry};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct SelectCandidate {
  config: Config,
  buffer: String,
  buffer_state: BufferState,
  dictionary_entry: DictionaryEntry,
  candidates: Candidates,
}

impl SelectCandidate {
  pub fn new(config: Config, dictionary_entry: &DictionaryEntry) -> Self {
    SelectCandidate {
      config,
      buffer: "".to_string(),
      buffer_state: BufferState::Continue,
      dictionary_entry: dictionary_entry.clone(),
      candidates: Candidates::new(&dictionary_entry.candidates),
    }
  }
}

impl WithConfig for SelectCandidate {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for SelectCandidate {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformable for SelectCandidate {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::SelectCandidate
  }

  fn try_change_transformer(&self, _: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    None
  }

  fn push_character(&self, _: char) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    Box::new(Canceled::new(self.config()))
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    let buffer = match self.candidates.current() {
      Some(candidate) => candidate.entry.clone(),
      None => "".to_string(),
    };

    Box::new(Stopped::new(self.config(), buffer))
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
      None => Box::new(Canceled::new(self.config())),
    }
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    self.push_delete()
  }
}

impl Displayable for SelectCandidate {
  fn buffer_content(&self) -> String {
    match self.candidates.current() {
      Some(v) => v.entry.clone(),
      None => "".to_string(),
    }
  }

  fn display_string(&self) -> String {
    match self.candidates.current() {
      Some(v) => "▼".to_string() + &v.entry,
      None => "".to_string(),
    }
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

impl AsTransformerTrait for SelectCandidate {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{key, set, Dictionary, RSKKConfig};
  use std::rc::Rc;

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
    let select_candidate = SelectCandidate::new(config.clone(), &dictionary_entry);

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
    let select_candidate = SelectCandidate::new(config.clone(), &dictionary_entry);

    let stopped = select_candidate.push_meta_key(&key!("enter"));
    assert_eq!(stopped.transformer_type(), TransformerTypes::Stopped);
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
    let select_candidate = SelectCandidate::new(config.clone(), &dictionary_entry);

    let select_candidate = select_candidate.push_meta_key(&key!("space"));
    let select_candidate = select_candidate.push_meta_key(&key!("delete"));
    assert_eq!(select_candidate.buffer_content(), "a");

    let canceled = select_candidate.push_meta_key(&key!("delete"));
    assert_eq!(canceled.transformer_type(), TransformerTypes::Canceled);
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

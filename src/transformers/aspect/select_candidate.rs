use super::super::{BufferState, Transformer, TransformerTypes};
use super::{Canceled, Stopped};
use crate::dictionary::{DictionaryEntry, TransformEntry};

#[derive(Clone, Debug)]
pub struct SelectCandidate {
  buffer: String,
  buffer_state: BufferState,
  dictionary_entry: DictionaryEntry,
  candidates: Candidates,
}

impl SelectCandidate {
  pub fn new(dictionary_entry: &DictionaryEntry) -> Self {
    SelectCandidate {
      buffer: "".to_string(),
      buffer_state: BufferState::Continue,
      dictionary_entry: dictionary_entry.clone(),
      candidates: Candidates::new(&dictionary_entry.transforms),
    }
  }
}

impl Transformer for SelectCandidate {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::SelectCandidate
  }

  fn is_stopped(&self) -> bool {
    self.buffer_state == BufferState::Stop
  }

  fn push(&mut self, _: char) -> Box<dyn Transformer> {
    Box::new(self.clone())
  }

  fn cancel(&mut self) -> Box<dyn Transformer> {
    self.buffer_state = BufferState::Stop;
    self.buffer = "".to_string();

    Box::new(Canceled::new())
  }

  fn enter(&mut self) -> Box<dyn Transformer> {
    self.buffer_state = BufferState::Stop;
    self.buffer = if let Some(candidate) = self.candidates.current() {
      candidate.entry.clone()
    } else {
      "".to_string()
    };

    Box::new(Stopped::new(self.buffer.clone()))
  }

  fn space(&mut self) -> Box<dyn Transformer> {
    let candidate = self.candidates.next();
    if candidate.is_none() {
      // TODO; 単語登録
      unimplemented!();
    }

    Box::new(self.clone())
  }

  fn delete(&mut self) -> Box<dyn Transformer> {
    let candidate = self.candidates.prev();
    if candidate.is_none() {
      return Box::new(Canceled::new());
    }

    Box::new(self.clone())
  }

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
  candidates: Vec<TransformEntry>,
  pos: usize,
}

impl Candidates {
  pub fn new(candidates: &Vec<TransformEntry>) -> Self {
    let mut items = Vec::new();
    for item in candidates.iter() {
      items.push(item.clone());
    }

    Candidates {
      candidates: items,
      pos: 0,
    }
  }

  pub fn next(&mut self) -> Option<&TransformEntry> {
    if self.pos >= self.candidates.len() {
      return None;
    }

    self.pos += 1;
    self.candidates.get(self.pos)
  }

  pub fn prev(&mut self) -> Option<&TransformEntry> {
    if self.pos <= 0 {
      return None;
    }

    self.pos -= 1;
    self.candidates.get(self.pos)
  }

  pub fn current(&self) -> Option<&TransformEntry> {
    self.candidates.get(self.pos)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn space() {
    let candidate1 = TransformEntry::new("a".to_string(), None);
    let candidate2 = TransformEntry::new("b".to_string(), None);
    let vec = vec![candidate1.clone(), candidate2.clone()];
    let dictionary_entry = DictionaryEntry::new("test".to_string(), vec);
    let mut select_candidate = SelectCandidate::new(&dictionary_entry);

    assert_eq!(select_candidate.buffer_content(), "a");
    assert_eq!(select_candidate.space().buffer_content(), "b");
    // TODO: 単語登録のテスト
  }

  #[test]
  fn enter() {
    let candidate1 = TransformEntry::new("a".to_string(), None);
    let vec = vec![candidate1.clone()];
    let dictionary_entry = DictionaryEntry::new("test".to_string(), vec);
    let mut select_candidate = SelectCandidate::new(&dictionary_entry);

    let stopped = select_candidate.enter();
    assert_eq!(stopped.transformer_type(), TransformerTypes::Stopped);
    assert_eq!(stopped.buffer_content(), "a");
  }

  #[test]
  fn delete() {
    let candidate1 = TransformEntry::new("a".to_string(), None);
    let candidate2 = TransformEntry::new("b".to_string(), None);
    let vec = vec![candidate1.clone(), candidate2.clone()];
    let dictionary_entry = DictionaryEntry::new("test".to_string(), vec);
    let mut select_candidate = SelectCandidate::new(&dictionary_entry);

    let mut select_candidate = select_candidate.space();
    let mut select_candidate = select_candidate.delete();
    assert_eq!(select_candidate.buffer_content(), "a");

    let canceled = select_candidate.delete();
    assert_eq!(canceled.transformer_type(), TransformerTypes::Canceled);
  }

  mod candidates {
    use super::*;

    #[test]
    fn prev() {
      let candidate1 = TransformEntry::new("a".to_string(), None);
      let candidate2 = TransformEntry::new("b".to_string(), None);
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

use super::StoppedTransformer;
use super::{
  AsTransformerTrait, BufferState, Config, Displayable, KeyCode, Stackable, Transformable,
  TransformerTypes, UnknownWordTransformer, WithConfig, Word,
};
use crate::dictionary::{Candidate, DictionaryEntry};

#[derive(Clone, Debug)]
pub struct SelectCandidateTransformer {
  config: Config,
  buffer: String,
  buffer_state: BufferState,
  dictionary_entry: DictionaryEntry,
  candidates: Candidates,
  word: Word,
}

impl SelectCandidateTransformer {
  pub fn new(config: Config, dictionary_entry: &DictionaryEntry, word: Word) -> Self {
    SelectCandidateTransformer {
      config,
      buffer: "".to_string(),
      buffer_state: BufferState::Continue,
      dictionary_entry: dictionary_entry.clone(),
      candidates: Candidates::new(&dictionary_entry.candidates),
      word,
    }
  }

  fn try_transition_to_stopped(&self) -> Option<Box<dyn Transformable>> {
    self
      .candidates
      .current()
      .and(Some(box StoppedTransformer::completed(
        self.config(),
        self.buffer_content(),
      )))
  }

  fn append_okuri(&self) -> Option<String> {
    self
      .candidates
      .current()
      .map(|candidate| candidate.entry.clone() + &self.word.okuri_string().unwrap_or("".to_owned()))
  }

  fn transition_to_unknown_word(&self) -> UnknownWordTransformer {
    UnknownWordTransformer::new(self.config(), self.word.clone())
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
      None => vec![box self.clone(), box self.transition_to_unknown_word()],
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
    self.append_okuri().unwrap_or("".to_owned())
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
  use super::super::tables::LetterType;
  use super::*;
  use crate::tds;
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::StoppedReason::*;
  use TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_conf();
    let candidate1 = Candidate::new("a", None);
    let candidate2 = Candidate::new("b", None);
    let vec = vec![candidate1.clone(), candidate2.clone()];
    let tf = SelectCandidateTransformer::new(
      conf.clone(),
      &DictionaryEntry::new("test", vec),
      Word::from((LetterType::Hiragana, "michigo")),
    );

    let items = tds![tf;
      ["", "▼a", SelectCandidate],
      ["[backspace]", "", Stopped(Canceled)],
      ["[escape]", "", Stopped(Canceled)],
      ["\n", "a", Stopped(Compleated)],
      [" ", "▼b", SelectCandidate],
      [" [backspace]", "▼a", SelectCandidate],
      ["  ", "[登録: みちご]", UnknownWord],
      ["  [escape]", "", Stopped(Canceled)],
      ["  a", "[登録: みちご]あ", UnknownWord],
      ["  a\n", "あ", Stopped(Compleated)],
    ];
    test_transformer(items);
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

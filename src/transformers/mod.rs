mod abbr;
mod continuous;
mod direct;
mod henkan;
mod hiragana;
mod katakana;
mod select_candidate;
mod stackable;
mod stopped;
mod tables;
mod unknown_word;
mod word;
mod yomi;

use objekt;
use std::fmt;

use crate::keyboards::{KeyCode, Keyboard, MetaKey};
use crate::{Context, DictionaryEntry};

pub use abbr::AbbrTransformer;
pub use continuous::ContinuousTransformer;
pub use direct::DirectTransformer;
pub use henkan::HenkanTransformer;
pub use hiragana::HiraganaTransformer;
pub use katakana::KatakanaTransformer;
pub use select_candidate::SelectCandidateTransformer;
pub use stackable::Stackable;
pub use stopped::{StoppedReason, StoppedTransformer};
pub use tables::LetterType;
pub use unknown_word::UnknownWordTransformer;
pub use word::Word;
pub use yomi::YomiTransformer;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum BufferState {
  Continue,
  Stop,
}

pub trait Displayable {
  fn buffer_content(&self) -> String;
  fn display_string(&self) -> String;
  fn is_empty(&self) -> bool {
    self.buffer_content().len() == 0
  }
}

pub trait WithContext {
  fn clone_context(&self) -> Context;
  fn context(&self) -> &Context;
  fn set_context(&mut self, context: Context);

  fn push_stopped_buffer(&self, buffer: String) -> Context {
    self.context().push_result_string(buffer)
  }

  fn clear_stopped_buffer(&self) -> Context {
    self.context().clear_stopped_buffer()
  }

  fn push_dictionary_updates(&mut self, updates: &Vec<DictionaryEntry>) -> Context {
    self.context().push_dictionary_updates(updates)
  }

  fn new_context(&self) -> Context {
    self.context().new_empty()
  }
}

pub trait Transformable:
  AsTransformerTrait + Displayable + Stackable + WithContext + objekt::Clone
{
  fn transformer_type(&self) -> TransformerTypes;
  fn is_base_transformer(&self) -> bool {
    match self.transformer_type() {
      TransformerTypes::Direct => true,
      TransformerTypes::Hiragana => true,
      TransformerTypes::Katakana => true,
      TransformerTypes::EnKatakana => true,
      TransformerTypes::EmEisu => true,
      _ => false,
    }
  }
  fn is_stopped(&self) -> bool {
    match self.transformer_type() {
      TransformerTypes::Stopped(_) => true,
      _ => false,
    }
  }

  fn is_compleated(&self) -> bool {
    match self.transformer_type() {
      TransformerTypes::Stopped(StoppedReason::Compleated) => true,
      _ => false,
    }
  }

  fn is_canceled(&self) -> bool {
    match self.transformer_type() {
      TransformerTypes::Stopped(StoppedReason::Canceled) => true,
      _ => false,
    }
  }

  fn to_completed(&self) -> Box<dyn Transformable> {
    box StoppedTransformer::completed(self.clone_context())
  }

  fn to_completed_with_update_buffer(&self, buffer: String) -> Box<dyn Transformable> {
    let context = self.push_stopped_buffer(buffer);

    box StoppedTransformer::completed(context)
  }

  fn to_canceled(&self) -> Box<dyn Transformable> {
    box StoppedTransformer::canceled(self.clone_context())
  }

  fn push_key(&self, key: &KeyCode) -> Option<Box<dyn Transformable>> {
    println!(
      "{}",
      format!("change transformer start {:?} {:?}", key, self.as_trait())
    );
    let ret = match (
      self.push_meta_key(key),
      key
        .printable_key()
        .and_then(|character| self.push_character(character)),
    ) {
      (Some(tfs), _) if tfs.is_empty() => Some(self.to_canceled()),
      (Some(tfs), _) => Some(tfs.last()?.clone()),
      (_, Some(tfs)) if tfs.is_empty() => Some(self.to_canceled()),
      (_, Some(tfs)) => Some(tfs.last()?.clone()),
      _ => None,
    };
    println!("{}", format!("change transformer end {:?} {:?}", key, &ret));

    ret
  }

  fn try_change_transformer(
    &self,
    _: &Box<dyn Keyboard>,
    _: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    None
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Option<Vec<Box<dyn Transformable>>> {
    match key_code {
      KeyCode::Meta(MetaKey::Escape) => self.push_escape(),
      KeyCode::PrintableMeta(MetaKey::Enter, _) | KeyCode::Meta(MetaKey::Enter) => {
        self.push_enter()
      }
      KeyCode::PrintableMeta(MetaKey::Space, _) | KeyCode::Meta(MetaKey::Space) => {
        self.push_space()
      }
      KeyCode::PrintableMeta(MetaKey::Backspace, _) | KeyCode::Meta(MetaKey::Backspace) => {
        self.push_backspace()
      }
      KeyCode::PrintableMeta(MetaKey::Delete, _) | KeyCode::Meta(MetaKey::Delete) => {
        self.push_delete()
      }
      KeyCode::PrintableMeta(MetaKey::Tab, _) | KeyCode::Meta(MetaKey::Tab) => self.push_tab(),
      KeyCode::Meta(MetaKey::ArrowRight) => self.push_arrow_right(),
      KeyCode::Meta(MetaKey::ArrowDown) => self.push_arrow_down(),
      KeyCode::Meta(MetaKey::ArrowLeft) => self.push_arrow_left(),
      KeyCode::Meta(MetaKey::ArrowUp) => self.push_arrow_up(),
      _ => self.push_any_character(key_code),
    }
  }
  fn push_character(&self, character: char) -> Option<Vec<Box<dyn Transformable>>>;

  fn push_escape(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_enter(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_space(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_backspace(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_delete(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_tab(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_null(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_arrow_right(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_arrow_down(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_arrow_left(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_arrow_up(&self) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
  fn push_any_character(&self, _: &KeyCode) -> Option<Vec<Box<dyn Transformable>>> {
    None
  }
}

impl dyn Transformable {
  #[allow(unused_must_use)]
  fn print_stack(&self, f: &mut fmt::Formatter<'_>, depth: usize) {
    let indent = "\t".repeat(depth);
    let stack = self.stack();
    if stack.len() == 0 {
      write!(
        f,
        "{}[{:?}: {}]",
        indent,
        self.transformer_type(),
        match &self.buffer_content() as &str {
          "" => "(empty)",
          some => some,
        }
      );
      return;
    }

    write!(
      f,
      "{}[{:?}: {}\n",
      indent,
      self.transformer_type(),
      self.buffer_content()
    );
    self.stack().iter().for_each(|s| {
      s.print_stack(f, depth + 1);
      write!(f, "\n");
    });
    write!(f, "{}]", indent);
  }
}

impl fmt::Debug for dyn Transformable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.print_stack(f, 0);

    fmt::Result::Ok(())
  }
}

pub trait AsTransformerTrait {
  fn as_trait(&self) -> Box<dyn Transformable>;

  fn send_target(&self) -> Box<dyn Transformable> {
    self.as_trait()
  }
}

objekt::clone_trait_object!(Transformable);

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum TransformerTypes {
  Direct,
  Hiragana,
  Katakana,
  Henkan,
  Okuri,
  Abbr,
  EmEisu,
  EnKatakana,
  Yomi,
  Stopped(StoppedReason),
  SelectCandidate,
  UnknownWord,
  Continuous,
  Letter(LetterType),
}

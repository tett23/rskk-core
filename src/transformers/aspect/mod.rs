mod canceled;
mod select_candidate;
mod stopped;
mod yomi;

use super::{Transformer, TransformerState, TransformerTypes};
use crate::keyboards::KeyCode;
use crate::{Config, Dictionary};
use std::collections::HashSet;
use std::rc::Rc;

pub use canceled::Canceled;
pub use select_candidate::SelectCandidate;
pub use stopped::Stopped;
pub use yomi::Yomi;

#[derive(Clone, Debug)]
pub enum Aspect {
  Yomi(Box<dyn Transformer>),
  // Okuri(Okuri),
  SelectCandidate(Box<dyn Transformer>),
  // UnknownWord(UnknownWord),
  Canceled(Box<dyn Transformer>),
  Stopped(Box<dyn Transformer>),
}

impl Aspect {
  pub fn new(
    config: Rc<Config>,
    dictionary: Rc<Dictionary>,
    transformer_type: TransformerTypes,
  ) -> Self {
    Aspect::Yomi(Box::new(Yomi::new(config, dictionary, transformer_type)))
  }
}

#[derive(Clone, Debug)]
pub struct AspectTransformer {
  aspect: Aspect,
  config: Rc<Config>,
  dictionary: Rc<Dictionary>,
}

// 未知語のとき単語登録はひらがなになる？
// 単語登録状態からカタカナ、abbrに遷移できる
impl AspectTransformer {
  pub fn new(
    config: Rc<Config>,
    dictionary: Rc<Dictionary>,
    transformer_type: TransformerTypes,
  ) -> Self {
    AspectTransformer {
      aspect: Aspect::new(config.clone(), dictionary.clone(), transformer_type),
      config,
      dictionary,
    }
  }
}

impl TransformerState for AspectTransformer {
  fn is_stopped(&self) -> bool {
    match &self.aspect {
      Aspect::Stopped(_) | Aspect::Canceled(_) => true,
      _ => false,
    }
  }
}

impl Transformer for AspectTransformer {
  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    match &self.aspect {
      Aspect::Yomi(t) => t.try_change_transformer(pressing_keys),
      Aspect::SelectCandidate(t) => t.try_change_transformer(pressing_keys),
      Aspect::Stopped(t) => t.try_change_transformer(pressing_keys),
      Aspect::Canceled(t) => t.try_change_transformer(pressing_keys),
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    println!("push character {}", character);
    let new_aspect = match &self.aspect {
      Aspect::Yomi(t) => t.push_character(character),
      Aspect::SelectCandidate(t) => t.push_character(character),
      Aspect::Stopped(t) => t.push_character(character),
      Aspect::Canceled(t) => t.push_character(character),
    };
    println!("new_aspect {:?}", new_aspect.buffer_content());
    let mut new_state = self.clone();
    new_state.aspect = match new_aspect.transformer_type() {
      TransformerTypes::Yomi => Aspect::Yomi(new_aspect),
      TransformerTypes::SelectCandidate => Aspect::SelectCandidate(new_aspect),
      TransformerTypes::Canceled => Aspect::Canceled(new_aspect),
      TransformerTypes::Stopped => Aspect::Stopped(new_aspect),
      _ => unreachable!(),
    };
    println!("new_state {:?}", new_state.buffer_content());

    Box::new(new_state)
  }

  fn push_key_code(&self, key_code: &KeyCode) -> Box<dyn Transformer> {
    let new_aspect = match &self.aspect {
      Aspect::Yomi(t) => t.push_key_code(key_code),
      Aspect::SelectCandidate(t) => t.push_key_code(key_code),
      Aspect::Stopped(t) => t.push_key_code(key_code),
      Aspect::Canceled(t) => t.push_key_code(key_code),
    };
    let mut new_state = self.clone();
    new_state.aspect = match new_aspect.transformer_type() {
      TransformerTypes::Yomi => Aspect::Yomi(new_aspect),
      TransformerTypes::SelectCandidate => Aspect::SelectCandidate(new_aspect),
      TransformerTypes::Canceled => Aspect::Canceled(new_aspect),
      TransformerTypes::Stopped => Aspect::Stopped(new_aspect),
      _ => unreachable!(),
    };

    Box::new(new_state)
  }

  fn buffer_content(&self) -> String {
    match &self.aspect {
      Aspect::Yomi(t) => t.buffer_content(),
      Aspect::SelectCandidate(t) => t.buffer_content(),
      Aspect::Stopped(t) => t.buffer_content(),
      Aspect::Canceled(t) => t.buffer_content(),
    }
  }

  fn display_string(&self) -> String {
    match &self.aspect {
      Aspect::Yomi(t) => t.display_string(),
      Aspect::SelectCandidate(t) => t.display_string(),
      Aspect::Stopped(t) => t.display_string(),
      Aspect::Canceled(t) => t.display_string(),
    }
  }
}

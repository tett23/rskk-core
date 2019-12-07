mod canceled;
mod continuous;
mod okuri_completed;
mod select_candidate;
mod stopped;
mod unknown_word;
mod yomi;

use super::{
  AsTransformerTrait, Config, Displayable, Transformable, TransformerState, TransformerTypes,
  WithConfig,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

pub use canceled::Canceled;
pub use continuous::ContinuousTransformer;
pub use okuri_completed::OkuriCompleted;
pub use select_candidate::SelectCandidate;
pub use stopped::Stopped;
pub use unknown_word::{UnknownWord, Word};
pub use yomi::YomiTransformer;

#[derive(Clone, Debug)]
pub enum Aspect {
  Yomi(Box<dyn Transformable>),
  // Okuri(Okuri),
  SelectCandidate(Box<dyn Transformable>),
  UnknownWord(Box<dyn Transformable>),
  Canceled(Box<dyn Transformable>),
  Stopped(Box<dyn Transformable>),
}

impl Aspect {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    Aspect::Yomi(Box::new(YomiTransformer::new(config, transformer_type)))
  }
}

#[derive(Clone, Debug)]
pub struct AspectTransformer {
  aspect: Aspect,
  config: Config,
}

// 未知語のとき単語登録はひらがなになる？
// 単語登録状態からカタカナ、abbrに遷移できる
impl AspectTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    AspectTransformer {
      aspect: Aspect::new(config.clone(), transformer_type),
      config,
    }
  }

  fn new_from_transformer(&self, aspect: Box<dyn Transformable>) -> Self {
    let mut ret = self.clone();
    ret.aspect = match aspect.transformer_type() {
      TransformerTypes::Yomi => Aspect::Yomi(aspect),
      TransformerTypes::SelectCandidate => Aspect::SelectCandidate(aspect),
      TransformerTypes::UnknownWord => Aspect::UnknownWord(aspect),
      TransformerTypes::Canceled => Aspect::Canceled(aspect),
      TransformerTypes::Stopped => Aspect::Stopped(aspect),
      _ => unreachable!(),
    };

    ret
  }
}

impl WithConfig for AspectTransformer {
  fn config(&self) -> Config {
    self.config.clone()
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

impl Transformable for AspectTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    match &self.aspect {
      Aspect::Yomi(t) => t.transformer_type(),
      Aspect::SelectCandidate(t) => t.transformer_type(),
      Aspect::UnknownWord(t) => t.transformer_type(),
      Aspect::Stopped(t) => t.transformer_type(),
      Aspect::Canceled(t) => t.transformer_type(),
    }
  }

  fn try_change_transformer(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    match &self.aspect {
      Aspect::Yomi(t) => t.try_change_transformer(pressing_keys, last_key_code),
      Aspect::SelectCandidate(t) => t.try_change_transformer(pressing_keys, last_key_code),
      Aspect::UnknownWord(t) => t.try_change_transformer(pressing_keys, last_key_code),
      Aspect::Stopped(t) => t.try_change_transformer(pressing_keys, last_key_code),
      Aspect::Canceled(t) => t.try_change_transformer(pressing_keys, last_key_code),
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let new_aspect = match &self.aspect {
      Aspect::Yomi(t) => t.push_character(character),
      Aspect::SelectCandidate(t) => t.push_character(character),
      Aspect::UnknownWord(t) => t.push_character(character),
      Aspect::Stopped(t) => t.push_character(character),
      Aspect::Canceled(t) => t.push_character(character),
    };

    Box::new(self.new_from_transformer(new_aspect))
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformable> {
    let new_aspect = match &self.aspect {
      Aspect::Yomi(t) => t.push_meta_key(key_code),
      Aspect::SelectCandidate(t) => t.push_meta_key(key_code),
      Aspect::UnknownWord(t) => t.push_meta_key(key_code),
      Aspect::Stopped(t) => t.push_meta_key(key_code),
      Aspect::Canceled(t) => t.push_meta_key(key_code),
    };

    Box::new(self.new_from_transformer(new_aspect))
  }
}

impl Displayable for AspectTransformer {
  fn buffer_content(&self) -> String {
    match &self.aspect {
      Aspect::Yomi(t) => t.buffer_content(),
      Aspect::SelectCandidate(t) => t.buffer_content(),
      Aspect::UnknownWord(t) => t.buffer_content(),
      Aspect::Stopped(t) => t.buffer_content(),
      Aspect::Canceled(t) => t.buffer_content(),
    }
  }

  fn display_string(&self) -> String {
    match &self.aspect {
      Aspect::Yomi(t) => t.display_string(),
      Aspect::SelectCandidate(t) => t.display_string(),
      Aspect::UnknownWord(t) => t.display_string(),
      Aspect::Stopped(t) => t.display_string(),
      Aspect::Canceled(t) => t.display_string(),
    }
  }
}

impl AsTransformerTrait for AspectTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

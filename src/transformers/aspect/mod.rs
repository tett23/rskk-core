mod canceled;
mod select_candidate;
mod stopped;
mod yomi;

use super::{Transformer, TransformerTypes};
use crate::{Config, Dictionary};
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

impl Transformer for AspectTransformer {
  fn is_stopped(&self) -> bool {
    match &self.aspect {
      Aspect::Yomi(t) => t.is_stopped(),
      Aspect::SelectCandidate(t) => t.is_stopped(),
      Aspect::Stopped(t) => t.is_stopped(),
      Aspect::Canceled(t) => t.is_stopped(),
    }
  }

  fn push(&mut self, character: char) -> Box<dyn Transformer> {
    let new_aspect = match &mut self.aspect {
      Aspect::Yomi(t) => t.push(character),
      Aspect::SelectCandidate(t) => t.push(character),
      Aspect::Stopped(t) => t.push(character),
      Aspect::Canceled(t) => t.push(character),
    };
    self.aspect = match new_aspect.transformer_type() {
      TransformerTypes::Yomi => Aspect::Yomi(new_aspect),
      TransformerTypes::SelectCandidate => Aspect::SelectCandidate(new_aspect),
      TransformerTypes::Canceled => Aspect::Canceled(new_aspect),
      TransformerTypes::Stopped => Aspect::Stopped(new_aspect),
      _ => unreachable!(),
    };

    Box::new(self.clone())
  }

  fn cancel(&mut self) -> Box<dyn Transformer> {
    let new_aspect = match &mut self.aspect {
      Aspect::Yomi(t) => t.cancel(),
      Aspect::SelectCandidate(t) => t.cancel(),
      Aspect::Stopped(t) => t.cancel(),
      Aspect::Canceled(t) => t.cancel(),
    };
    self.aspect = match new_aspect.transformer_type() {
      TransformerTypes::Yomi => Aspect::Yomi(new_aspect),
      TransformerTypes::SelectCandidate => Aspect::SelectCandidate(new_aspect),
      TransformerTypes::Canceled => Aspect::Canceled(new_aspect),
      TransformerTypes::Stopped => Aspect::Stopped(new_aspect),
      _ => unreachable!(),
    };

    Box::new(self.clone())
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
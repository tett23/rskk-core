use super::super::{BufferState, Transformer, TransformerState, TransformerTypes};
use super::{Canceled, Stopped};
use crate::keyboards::{KeyCode, MetaKey};
use crate::{Config, Dictionary};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Yomi {
  config: Rc<Config>,
  dictionary: Rc<Dictionary>,
  buffer: String,
  buffer_state: BufferState,
  transformer_type: TransformerTypes,
  transformer: Box<dyn Transformer>,
}

impl Yomi {
  pub fn new(
    config: Rc<Config>,
    dictionary: Rc<Dictionary>,
    transformer_type: TransformerTypes,
  ) -> Self {
    Yomi {
      config: config.clone(),
      dictionary: dictionary.clone(),
      buffer: "".to_string(),
      buffer_state: BufferState::Continue,
      transformer_type,
      transformer: transformer_type.to_transformer(config.clone(), dictionary.clone()),
    }
  }

  pub fn new_from_buffer(yomi: &Self, buffer: String) -> Self {
    Yomi {
      config: yomi.config.clone(),
      dictionary: yomi.dictionary.clone(),
      buffer: buffer.clone(),
      buffer_state: yomi.buffer_state.clone(),
      transformer_type: yomi.transformer_type.clone(),
      transformer: yomi
        .transformer_type
        .to_transformer(yomi.config.clone(), yomi.dictionary.clone()),
    }
  }

  pub fn new_from_transformer(yomi: &Self, transformer: Box<dyn Transformer>) -> Self {
    Yomi {
      config: yomi.config.clone(),
      dictionary: yomi.dictionary.clone(),
      buffer: yomi.buffer.clone(),
      buffer_state: yomi.buffer_state.clone(),
      transformer_type: yomi.transformer_type.clone(),
      transformer: transformer,
    }
  }
}

impl TransformerState for Yomi {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformer for Yomi {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Yomi
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    if self.buffer_state == BufferState::Stop {
      return Box::new(Stopped::new(self.buffer.clone()));
    }

    let new_transformer = self.transformer.push_character(character);
    if new_transformer.is_stopped() {
      let new_buffer = self.buffer.clone() + &new_transformer.buffer_content();
      return Box::new(Self::new_from_buffer(self, new_buffer));
    }

    Box::new(Self::new_from_transformer(self, new_transformer))
  }

  fn push_key_code(&self, _: &HashSet<KeyCode>, key_code: &KeyCode) -> Box<dyn Transformer> {
    match key_code {
      KeyCode::Meta(MetaKey::Escape) => Box::new(Canceled::new()),
      KeyCode::PrintableMeta(MetaKey::Enter, _) | KeyCode::Meta(MetaKey::Enter) => {
        // TODO: たぶんstop
        unimplemented!()
      }
      KeyCode::PrintableMeta(MetaKey::Space, _) | KeyCode::Meta(MetaKey::Space) => {
        // TODO: bufferかtransformerから文字を削除
        unimplemented!();
      }
      KeyCode::PrintableMeta(MetaKey::Tab, _) | KeyCode::Meta(MetaKey::Tab) => {
        // TODO: 補完して新しいYomiTransformerを返す
        unimplemented!()
      }
      _ => Box::new(self.clone()),
    }
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    if self.buffer.len() == 0 {
      return "".to_string();
    }

    "▽".to_string() + &self.buffer + &self.transformer.display_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  #[test]
  fn push() {
    let config = Rc::new(Config::default_config());
    let dictionary = Rc::new(Dictionary::new(HashSet::new()));
    let yomi = Yomi::new(config, dictionary, TransformerTypes::Hiragana);

    let yomi = yomi.push_character('a');
    assert_eq!(yomi.buffer_content(), "あ");

    let yomi = yomi.push_character('k');
    assert_eq!(yomi.display_string(), "▽あk");

    let yomi = yomi.push_character('a');
    assert_eq!(yomi.buffer_content(), "あか");
  }
}

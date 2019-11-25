use super::super::{SelectCandidate, Transformer, TransformerState, TransformerTypes};
use super::Canceled;
use crate::keyboards::{KeyCode, MetaKey};
use crate::{Config, Dictionary};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Yomi {
  config: Rc<Config>,
  dictionary: Rc<Dictionary>,
  buffer: String,
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
      transformer_type,
      transformer: transformer_type.to_transformer(config.clone(), dictionary.clone()),
    }
  }

  pub fn new_from_buffer(&self, buffer: String) -> Self {
    let mut ret = self.clone();
    ret.buffer = buffer;

    ret
  }

  pub fn new_from_transformer(&self, transformer: Box<dyn Transformer>) -> Self {
    let mut ret = self.clone();
    ret.transformer = transformer;

    ret
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

  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self.transformer.try_change_transformer(pressing_keys)
  }

  fn push_character(&self, character: char) -> Box<dyn Transformer> {
    println!("push yomi {}", character);
    // ここらへんCompositionの再利用でよさそう
    let new_transformer = self.transformer.push_character(character);
    if new_transformer.is_stopped() {
      let new_buffer = self.buffer.clone() + &new_transformer.buffer_content();
      return Box::new(self.new_from_buffer(new_buffer));
    }

    println!("new_transformer {:?}", new_transformer.buffer_content());
    let ret = Box::new(self.new_from_transformer(new_transformer));
    println!("new_transformer {:?}", ret.buffer_content());
    ret
  }

  fn push_key_code(&self, key_code: &KeyCode) -> Box<dyn Transformer> {
    match key_code {
      KeyCode::Meta(MetaKey::Escape) => Box::new(Canceled::new()),
      KeyCode::PrintableMeta(MetaKey::Enter, _) | KeyCode::Meta(MetaKey::Enter) => {
        // TODO: たぶんstop
        unimplemented!()
      }
      KeyCode::PrintableMeta(MetaKey::Space, _) | KeyCode::Meta(MetaKey::Space) => {
        match self.dictionary.transform(&self.buffer_content()) {
          Some(entry) => Box::new(SelectCandidate::new(entry)),
          None => {
            // TODO: 単語登録
            unimplemented!()
          }
        }
      }
      KeyCode::PrintableMeta(MetaKey::Backspace, _)
      | KeyCode::Meta(MetaKey::Backspace)
      | KeyCode::PrintableMeta(MetaKey::Delete, _)
      | KeyCode::Meta(MetaKey::Delete) => {
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
    self.buffer.clone() + &self.transformer.buffer_content()
  }

  fn display_string(&self) -> String {
    let buf = self.buffer.clone() + &self.transformer.display_string();
    if buf.len() == 0 {
      return buf;
    }

    "▽".to_string() + &buf
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

    // let yomi = yomi.push_character('a');
    // assert_eq!(yomi.buffer_content(), "あか");
  }
}

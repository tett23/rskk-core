use super::super::{
  AsTransformerTrait, Canceled, Config, Displayable, SelectCandidate, Stopped, Transformable,
  TransformerState, TransformerTypes, UnknownWord, WithConfig,
};
use super::unknown_word::Word;
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct YomiTransformer {
  config: Config,
  buffer: String,
  transformer_type: TransformerTypes,
  transformer: Box<dyn Transformable>,
}

impl YomiTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    YomiTransformer {
      config: config.clone(),
      buffer: "".to_string(),
      transformer_type,
      transformer: transformer_type.to_transformer(config),
    }
  }

  pub fn new_from_buffer(&self, buffer: String) -> Self {
    let mut ret = self.clone();
    ret.buffer = buffer;
    ret.transformer = self.transformer_type.to_transformer(self.config());

    ret
  }

  pub fn new_from_transformer(&self, transformer: Box<dyn Transformable>) -> Self {
    let mut ret = self.clone();
    ret.transformer = transformer;

    ret
  }
}

impl WithConfig for YomiTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for YomiTransformer {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformable for YomiTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Yomi
  }

  fn try_change_transformer(&self, pressing_keys: &HashSet<KeyCode>) -> Option<TransformerTypes> {
    self.transformer.try_change_transformer(pressing_keys)
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    // ここらへんCompositionの再利用でよさそう
    let new_transformer = self.transformer.push_character(character);

    match new_transformer.is_stopped() {
      true => {
        let new_buffer = self.buffer.clone() + &new_transformer.buffer_content();
        Box::new(self.new_from_buffer(new_buffer))
      }
      false => Box::new(self.new_from_transformer(new_transformer)),
    }
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    Box::new(Canceled::new(self.config()))
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    Box::new(Stopped::new(self.config(), self.buffer_content()))
  }

  fn push_space(&self) -> Box<dyn Transformable> {
    match self.config().dictionary.transform(&self.buffer_content()) {
      Some(entry) => Box::new(SelectCandidate::new(self.config(), entry)),
      None => Box::new(UnknownWord::new(
        self.config(),
        Word::new(self.buffer_content(), None),
      )),
    }
  }

  fn push_delete(&self) -> Box<dyn Transformable> {
    // TODO: bufferかtransformerから文字を削除
    unimplemented!();
  }

  fn push_backspace(&self) -> Box<dyn Transformable> {
    self.push_delete()
  }

  fn push_tab(&self) -> Box<dyn Transformable> {
    // TODO: 補完して新しいYomiTransformerを返す
    unimplemented!()
  }
}

impl Displayable for YomiTransformer {
  fn buffer_content(&self) -> String {
    self.buffer.clone() + &self.transformer.buffer_content()
  }

  fn display_string(&self) -> String {
    "▽".to_string() + &self.buffer_content()
  }
}

impl AsTransformerTrait for YomiTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::dummy_conf;

  #[test]
  fn push() {
    let config = dummy_conf();
    let yomi = Yomi::new(config.clone(), TransformerTypes::Hiragana);

    let yomi = yomi.push_character('a');
    assert_eq!(yomi.display_string(), "▽あ");

    let yomi = yomi.push_character('k');
    assert_eq!(yomi.display_string(), "▽あk");

    let yomi = yomi.push_character('a');
    assert_eq!(yomi.display_string(), "▽あか");
  }
}

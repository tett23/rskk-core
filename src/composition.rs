use super::keyboards::{KeyEvents, Keyboard};
use super::transformers::{Config, Transformable, TransformerTypes};
use crate::tf;

pub struct Composition {
  transformer: Box<dyn Transformable>,
  keyboard: Box<dyn Keyboard>,
  // TODO: 変更のあった辞書要素を保持できる必要あり？
  // 変更は読みと変換先だけあればいいかな。
  // 読みがマッチした要素の候補の先頭に候補を挿入する
  // 未登録の場合は新規レコードを追加
}

impl Composition {
  pub fn new(config: Config, transformer_types: TransformerTypes) -> Self {
    let keyboard = config.rskk_config().keyboard_type.to_keyboard();

    Composition {
      transformer: tf!(config, transformer_types),
      keyboard: keyboard,
    }
  }

  #[cfg(test)]
  pub fn new_from_transformer(config: Config, transformer: Box<dyn Transformable>) -> Self {
    let keyboard = config.rskk_config().keyboard_type.to_keyboard();

    Composition {
      transformer,
      keyboard: keyboard,
    }
  }

  pub fn transformer_type(&self) -> TransformerTypes {
    self.transformer.transformer_type()
  }

  pub fn push_key_events(&mut self, events: &Vec<KeyEvents>) {
    events.iter().for_each(|e| self.push_key_event(e))
  }

  pub fn push_key_event(&mut self, event: &KeyEvents) {
    self.keyboard.push_event(event);
    self.transformer = self.transformer.push_key_event(&self.keyboard, event);
  }

  pub fn buffer_content(&self) -> String {
    self.transformer.buffer_content()
  }

  pub fn display_string(&self) -> String {
    self.transformer.display_string()
  }

  #[cfg(test)]
  pub fn transformer(&self) -> Box<dyn Transformable> {
    self.transformer.clone()
  }
}

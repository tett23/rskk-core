use super::keyboards::{KeyCode, KeyEvents, Keyboard, MetaKey};
use super::transformers::{Config, Transformable, TransformerTypes};
use crate::tf;

#[derive(Clone)]
pub struct Composition {
  stack: Vec<Box<dyn Transformable>>,
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
      stack: vec![tf!(config, transformer_types)],
      keyboard: keyboard,
    }
  }

  #[cfg(test)]
  pub fn new_from_transformer(config: Config, transformer: Box<dyn Transformable>) -> Self {
    let keyboard = config.rskk_config().keyboard_type.to_keyboard();

    Composition {
      stack: vec![transformer],
      keyboard: keyboard,
    }
  }

  pub fn is_stopped(&self) -> bool {
    self.stack.iter().all(|tf| tf.is_stopped())
  }

  pub fn is_empty(&self) -> bool {
    self.stack.iter().all(|tf| tf.is_empty())
  }

  pub fn push_key_events(&mut self, events: &Vec<KeyEvents>) {
    events.iter().for_each(|e| {
      self.push_key_event(e);
    })
  }

  pub fn push_key_event(&mut self, event: &KeyEvents) -> bool {
    self.keyboard.push_event(event);
    if let (KeyEvents::KeyDown(KeyCode::Meta(MetaKey::Delete)), true) = (event, self.is_empty()) {
      return false;
    }

    let new_tf = self
      .stack
      .last()
      .map(|tf| tf.push_key_event(&self.keyboard, event));
    if new_tf.is_none() {
      return false;
    }
    self.stack.pop();
    self.stack.push(new_tf.unwrap());

    true
  }

  pub fn buffer_content(&self) -> String {
    self
      .stack
      .iter()
      .fold("".to_string(), |acc, item| acc + &item.buffer_content())
  }

  pub fn display_string(&self) -> String {
    self
      .stack
      .iter()
      .fold("".to_string(), |acc, item| acc + &item.display_string())
  }

  #[cfg(test)]
  pub fn transformer_type(&self) -> TransformerTypes {
    self.stack.last().unwrap().transformer_type()
  }

  #[cfg(test)]
  pub fn transformer(&self) -> Box<dyn Transformable> {
    self.stack.last().unwrap().clone()
  }
}

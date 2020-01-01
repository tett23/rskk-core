use super::keyboards::{KeyCode, KeyEvents, Keyboard, MetaKey};
use super::transformers::{Config, Transformable, TransformerTypes};
use crate::tf;

#[derive(Clone)]
pub struct Composition {
  transformer: Box<dyn Transformable>,
  base_transformer_type: TransformerTypes,
  config: Config,
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
      transformer: tf!(config.clone(), transformer_types),
      base_transformer_type: transformer_types,
      config: config,
      keyboard: keyboard,
    }
  }

  #[cfg(test)]
  pub fn new_from_transformer(config: Config, transformer: Box<dyn Transformable>) -> Self {
    let keyboard = config.rskk_config().keyboard_type.to_keyboard();

    Composition {
      transformer,
      base_transformer_type: TransformerTypes::Direct,
      config: config,
      keyboard: keyboard,
    }
  }

  pub fn is_stopped(&self) -> bool {
    self.transformer.is_stopped()
  }

  pub fn is_empty(&self) -> bool {
    self.transformer.is_empty()
  }

  pub fn push_key_events(&mut self, events: &Vec<KeyEvents>) {
    events.iter().for_each(|e| {
      self.push_key_event(e);
    })
  }

  pub fn push_key_event(&mut self, event: &KeyEvents) -> bool {
    self.keyboard.push_event(event);
    dbg!(&self.transformer);

    match event {
      KeyEvents::KeyDown(KeyCode::Meta(MetaKey::Delete)) if self.is_empty() => None,
      KeyEvents::KeyDown(_) => self.keyboard.last_character(),
      _ => None,
    }
    .map(|key| {
      if let Some(new_tf) = self.try_change_transformer(&self.keyboard, &key) {
        self.base_transformer_type = new_tf.transformer_type();
        self.transformer = new_tf;
        true
      } else {
        self.transformer = self.transformer.push_key(&key);
        true
      }
    })
    .unwrap_or(false)
  }

  pub fn next_composition(&self) -> Composition {
    Composition::new(self.config.clone(), self.base_transformer_type)
  }

  fn try_change_transformer(
    &self,
    keyboard: &Box<dyn Keyboard>,
    key: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    self.transformer().try_change_transformer(keyboard, key)
  }

  pub fn buffer_content(&self) -> String {
    self.transformer.buffer_content()
  }

  pub fn display_string(&self) -> String {
    self.transformer.display_string()
  }

  pub fn transformer_type(&self) -> TransformerTypes {
    self.transformer.transformer_type()
  }

  pub fn child_transformer_type(&self) -> TransformerTypes {
    self.transformer.child_transformer_type()
  }

  pub fn transformer(&self) -> Box<dyn Transformable> {
    self.transformer.clone()
  }

  pub fn base_transformer_type(&self) -> TransformerTypes {
    self.base_transformer_type
  }
}

use super::keyboards::{KeyCode, KeyEvents, Keyboard};
use super::transformers::{Transformer, TransformerState, TransformerTypes};
use crate::{Config, Dictionary};
use std::rc::Rc;

pub struct Composition {
  config: Rc<Config>,
  dictionary: Rc<Dictionary>,
  buffer: String,
  transformer: Box<dyn Transformer>,
  current_transformer_type: TransformerTypes,
  keyboard: Box<dyn Keyboard>,
  // TODO: 変更のあった辞書要素を保持できる必要あり？
  // 変更は読みと変換先だけあればいいかな。
  // 読みがマッチした要素の候補の先頭に候補を挿入する
  // 未登録の場合は新規レコードを追加
}

impl Composition {
  pub fn new(
    config: Rc<Config>,
    dictionary: Rc<Dictionary>,
    transformer_types: TransformerTypes,
  ) -> Self {
    let keyboard = config.keyboard_type.to_keyboard();

    Composition {
      config: config.clone(),
      dictionary: dictionary.clone(),
      buffer: "".to_string(),
      transformer: transformer_types.to_transformer(config.clone(), dictionary.clone()),
      current_transformer_type: transformer_types,
      keyboard: keyboard,
    }
  }

  pub fn push_key_events(&mut self, events: &Vec<KeyEvents>) {
    events.iter().for_each(|e| self.push_key_event(e))
  }

  pub fn push_key_event(&mut self, event: &KeyEvents) {
    self.keyboard.push_event(event);

    match event {
      KeyEvents::KeyDown(key) => {
        if let Some(new_transformer_type) = self.try_change_transformer() {
          self.current_transformer_type = new_transformer_type;
          self.replace_new_transformer();
          println!("new_transformer_type {:?}", new_transformer_type);

          match new_transformer_type {
            TransformerTypes::Henkan => {
              if let Some(character) = key.printable_key() {
                self.push_character(character)
              };
            }
            _ => {}
          };
          return;
        };

        self.push_meta_key(key);
        if let Some(character) = self.keyboard.last_character() {
          self.push_character(character)
        };
      }
      KeyEvents::KeyUp(_) => {}
      KeyEvents::KeyRepeat(_) => unimplemented!(),
    }
  }

  fn try_change_transformer(&mut self) -> Option<TransformerTypes> {
    let new_transformer_type = self
      .transformer
      .try_change_transformer(self.keyboard.pressing_keys());
    if new_transformer_type.is_none() {
      return None;
    }
    let new_transformer_type = new_transformer_type.unwrap();

    self.current_transformer_type = new_transformer_type;

    Some(new_transformer_type)
  }

  fn push_meta_key(&mut self, key_code: &KeyCode) {
    self.transformer = self.transformer.push_key_code(key_code);
    if self.transformer.is_stopped() {
      self.replace_new_transformer();
    }
  }

  fn push_character(&mut self, character: char) {
    self.transformer = self.transformer.push_character(character);
    println!(
      "push_character result {:?}",
      self.transformer.buffer_content()
    );
    println!(
      "push_character result {:?}",
      self.transformer.display_string()
    );
    if self.transformer.is_stopped() {
      self.replace_new_transformer();
    }
  }

  pub fn buffer_content(&self) -> String {
    self.buffer.clone() + &self.transformer.buffer_content()
  }

  pub fn display_string(&self) -> String {
    self.buffer.clone() + &self.transformer.display_string()
  }

  fn replace_new_transformer(&mut self) {
    self.buffer = self.buffer_content();
    // TODO: あとで空のStoppedにする
    self.transformer = self
      .current_transformer_type
      .to_transformer(self.config.clone(), self.dictionary.clone());
  }
}

impl TransformerState for Composition {
  fn is_stopped(&self) -> bool {
    self.transformer.is_stopped()
  }
}

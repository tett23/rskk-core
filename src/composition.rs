use super::config::Config;
use super::keyboards::{KeyCode, Keyboard};
use super::transformers::{Transformer, TransformerTypes};
use std::rc::Rc;

pub struct Composition {
  config: Rc<Config>,
  transformer: Box<dyn Transformer>,
  current_transformer_type: TransformerTypes,
  compositioned_buffer: String,
  keyboard: Box<dyn Keyboard>,
}

impl Composition {
  pub fn new(config: Rc<Config>, transformer_types: TransformerTypes) -> Self {
    let keyboard = config.keyboard_type.to_keyboard();

    Composition {
      config,
      transformer: transformer_types.to_transformer(),
      current_transformer_type: transformer_types,
      compositioned_buffer: "".to_string(),
      keyboard: keyboard,
    }
  }

  pub fn key_down(&mut self, key_code: &KeyCode) {
    let character = self.keyboard.key_down(key_code);
    if self.try_replace_transformer() {
      return;
    }

    if let Some(character) = character {
      self.push_character(character)
    }
  }

  pub fn key_up(&mut self, key: &KeyCode) {
    self.keyboard.key_up(key);
  }

  fn try_replace_transformer(&mut self) -> bool {
    let new_transformer_type = self
      .keyboard
      .try_change_transformer(&self.config.key_config, &self.current_transformer_type);
    if new_transformer_type.is_none() {
      return false;
    }
    let new_transformer_type = new_transformer_type.unwrap();

    self.replace_transfomer(new_transformer_type);

    true
  }

  fn replace_transfomer(&mut self, replace_to: TransformerTypes) {
    self
      .compositioned_buffer
      .push_str(&self.transformer.buffer_content());

    self.current_transformer_type = replace_to;
    std::mem::replace(&mut self.transformer, replace_to.to_transformer());
  }

  pub fn push_character(&mut self, character: char) {
    self.transformer.push(character);
    if !self.transformer.is_stopped() {
      return;
    }

    self.replace_transfomer(self.current_transformer_type);
  }

  pub fn display_string(&self) -> String {
    let mut ret = "".to_string();
    ret.push_str(&self.compositioned_buffer);
    ret.push_str(&self.transformer.display_string());

    ret
  }
}

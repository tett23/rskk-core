use crate::composition_buffer::CompositionBuffer;
use crate::keyboards;
use crate::keycodes::KeyCode;
use crate::transformers::TransformerTypes;

pub struct Composition {
  composition_buffer: CompositionBuffer,
  keyboard: Box<dyn keyboards::Keyboard>,
}

impl Composition {
  pub fn new(keyboard_type: &keyboards::Keyboards, transformer_type: TransformerTypes) -> Self {
    Composition {
      composition_buffer: CompositionBuffer::new(transformer_type),
      keyboard: keyboard_type.to_keyboard(),
    }
  }

  pub fn key_down(&mut self, key_code: &KeyCode) {
    let character = self.keyboard.key_down(key_code);
    if let Some(c) = character {
      self.composition_buffer.push_character(c);
    }
  }

  pub fn key_up(&mut self, key: &KeyCode) {
    self.keyboard.key_up(key);
  }

  pub fn display_string(&self) -> String {
    self.composition_buffer.display_string()
  }
}

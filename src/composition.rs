use crate::composition_buffer::CompositionBuffer;
use crate::composition_types::CompositionType;
use crate::keycodes::KeyCode;
use std::collections::HashSet;

pub struct Composition {
  composition_buffer: CompositionBuffer,
  pressing_keys: HashSet<KeyCode>,
}

impl Composition {
  pub fn new(composition_type: CompositionType) -> Self {
    Composition {
      composition_buffer: CompositionBuffer::new(composition_type),
      pressing_keys: HashSet::new(),
    }
  }

  pub fn key_down(&mut self, key: &KeyCode) {
    // 絵文字直接入力に対応するため、コードポイントを渡せるようにしたほうがいい？
    // non asciiなキーボード時に記号入力がおかしくなりそう
    // キーボードの抽象化の層が必要では？
    self.pressing_keys.insert(key.clone());
    self
      .composition_buffer
      .push_key(key, self.is_pressing_shift());
  }

  fn is_pressing_shift(&self) -> bool {
    self.is_pressing(&KeyCode::Shift)
  }

  // fn is_pressing_ctrl(&self) -> bool {
  //   self.is_pressing(&KeyCode::Shift)
  // }

  fn is_pressing(&self, key: &KeyCode) -> bool {
    self.pressing_keys.contains(key)
  }

  pub fn key_up(&mut self, key: &KeyCode) {
    self.pressing_keys.remove(key);
  }

  pub fn display_string(&self) -> String {
    self.composition_buffer.display_string()
  }
}

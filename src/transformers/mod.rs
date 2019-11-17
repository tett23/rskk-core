pub mod direct;

use crate::keycodes::KeyCode;

pub trait Transformer {
  fn is_stopped(&self) -> bool;
  fn push(&mut self, key: &KeyCode, shift: bool);
  fn exit(&mut self) -> String;
  fn buffer_content(&self) -> String;
  fn display_string(&self) -> String;
}

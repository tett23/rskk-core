pub mod direct;

pub trait Transformer {
  fn is_stopped(&self) -> bool;
  fn push(&mut self, character: char);
  fn exit(&mut self) -> String;
  fn buffer_content(&self) -> String;
  fn display_string(&self) -> String;
}

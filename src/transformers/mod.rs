mod direct;
mod hiragana;
mod tables;

pub type DirectTransformer = direct::DirectTransformer;
pub type HiraganaTransformer = hiragana::HiraganaTransformer;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BufferState {
  Continue,
  Stop,
}

pub trait Transformer {
  fn is_stopped(&self) -> bool;
  fn push(&mut self, character: char);
  fn exit(&mut self) -> String;
  fn buffer_content(&self) -> String;
  fn display_string(&self) -> String;
}

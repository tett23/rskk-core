use super::super::{Transformer, TransformerTypes};

#[derive(Clone, Debug)]
pub struct Stopped {
  buffer: String,
}

impl Stopped {
  pub fn new(buffer: String) -> Self {
    Stopped { buffer }
  }
}

impl Transformer for Stopped {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Stopped
  }

  fn is_stopped(&self) -> bool {
    true
  }

  fn push(&mut self, _: char) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.buffer_content()))
  }

  fn cancel(&mut self) -> Box<dyn Transformer> {
    Box::new(Stopped::new("".to_string()))
  }

  fn enter(&mut self) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.buffer_content()))
  }

  fn tab(&mut self) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.buffer_content()))
  }

  fn space(&mut self) -> Box<dyn Transformer> {
    Box::new(Stopped::new(self.buffer_content()))
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

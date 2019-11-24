use super::super::{Transformer, TransformerTypes};

#[derive(Clone, Debug)]
pub struct Canceled {}

impl Canceled {
  pub fn new() -> Self {
    Canceled {}
  }
}

impl Transformer for Canceled {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Canceled
  }

  fn is_stopped(&self) -> bool {
    true
  }

  fn push(&mut self, _: char) -> Box<dyn Transformer> {
    Box::new(Canceled::new())
  }

  fn cancel(&mut self) -> Box<dyn Transformer> {
    Box::new(Canceled::new())
  }

  fn enter(&mut self) -> Box<dyn Transformer> {
    Box::new(Canceled::new())
  }

  fn tab(&mut self) -> Box<dyn Transformer> {
    Box::new(Canceled::new())
  }

  fn space(&mut self) -> Box<dyn Transformer> {
    Box::new(Canceled::new())
  }

  fn buffer_content(&self) -> String {
    "".to_string()
  }

  fn display_string(&self) -> String {
    "".to_string()
  }
}

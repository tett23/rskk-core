use crate::transformers::Transformer;

pub struct DirectTransformer {
  buffer: String,
  is_stopped: bool,
}

impl DirectTransformer {
  pub fn new() -> Self {
    DirectTransformer {
      buffer: "".to_string(),
      is_stopped: false,
    }
  }
}

impl Transformer for DirectTransformer {
  fn is_stopped(&self) -> bool {
    self.is_stopped
  }

  fn push(&mut self, character: char) {
    if self.is_stopped {
      return;
    }

    self.is_stopped = true;
    self.buffer.push(character);
  }

  fn exit(&mut self) -> String {
    self.is_stopped = true;

    std::mem::replace(&mut self.buffer, "".to_string())
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}

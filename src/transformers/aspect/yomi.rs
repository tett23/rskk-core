use super::super::{BufferState, Transformer, TransformerTypes};
use super::Stopped;

#[derive(Clone, Debug)]
pub struct Yomi {
  buffer: String,
  buffer_state: BufferState,
  transformer_type: TransformerTypes,
  transformer: Box<dyn Transformer>,
}

impl Yomi {
  pub fn new(transformer_type: TransformerTypes) -> Self {
    Yomi {
      buffer: "".to_string(),
      buffer_state: BufferState::Continue,
      transformer_type,
      transformer: transformer_type.to_transformer(),
    }
  }

  pub fn new_from_buffer(yomi: &Self, buffer: String) -> Self {
    Yomi {
      buffer: buffer.clone(),
      buffer_state: yomi.buffer_state.clone(),
      transformer_type: yomi.transformer_type.clone(),
      transformer: yomi.transformer_type.to_transformer(),
    }
  }

  pub fn new_from_transformer(yomi: &Self, transformer: Box<dyn Transformer>) -> Self {
    Yomi {
      buffer: yomi.buffer.clone(),
      buffer_state: yomi.buffer_state.clone(),
      transformer_type: yomi.transformer_type.clone(),
      transformer: transformer,
    }
  }
}

impl Transformer for Yomi {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Yomi
  }

  fn is_stopped(&self) -> bool {
    self.buffer_state == BufferState::Stop
  }

  fn push(&mut self, character: char) -> Box<dyn Transformer> {
    if self.buffer_state == BufferState::Stop {
      return Box::new(Stopped::new(self.buffer.clone()));
    }

    let new_transformer = self.transformer.push(character);
    if new_transformer.is_stopped() {
      let new_buffer = self.buffer.clone() + &new_transformer.buffer_content();
      return Box::new(Self::new_from_buffer(self, new_buffer));
    }

    Box::new(Self::new_from_transformer(self, new_transformer))
  }

  fn cancel(&mut self) -> Box<dyn Transformer> {
    self.buffer_state = BufferState::Stop;
    self.buffer = "".to_string();

    Box::new(Stopped::new("".to_string()))
  }

  fn enter(&mut self) -> Box<dyn Transformer> {
    self.buffer_state = BufferState::Stop;

    Box::new(Stopped::new(self.buffer_content()))
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone() + &self.transformer.buffer_content()
  }

  fn display_string(&self) -> String {
    if self.buffer.len() == 0 {
      return "".to_string();
    }

    "▽".to_string() + &self.buffer + &self.transformer.display_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn push() {
    let mut yomi = Yomi::new(TransformerTypes::Hiragana);

    let mut yomi = yomi.push('a');
    assert_eq!(yomi.buffer_content(), "あ");

    let mut yomi = yomi.push('k');
    assert_eq!(yomi.buffer_content(), "あk");

    let yomi = yomi.push('a');
    assert_eq!(yomi.buffer_content(), "あか");
  }
}

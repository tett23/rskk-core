use crate::transformers::{Transformer, TransformerTypes};

pub struct CompositionBuffer {
  transformer: Box<dyn Transformer>,
  current_transformer_type: TransformerTypes,
  compositioned_buffer: String,
}

impl CompositionBuffer {
  pub fn new(transformer_types: TransformerTypes) -> Self {
    CompositionBuffer {
      transformer: transformer_types.to_transformer(),
      current_transformer_type: transformer_types,
      compositioned_buffer: "".to_string(),
    }
  }

  pub fn push_character(&mut self, character: char) {
    self.transformer.push(character);
    if !self.transformer.is_stopped() {
      return;
    }

    self
      .compositioned_buffer
      .push_str(&self.transformer.buffer_content());

    std::mem::replace(
      &mut self.transformer,
      self.current_transformer_type.to_transformer(),
    );
  }

  pub fn display_string(&self) -> String {
    let mut ret = "".to_string();
    ret.push_str(&self.compositioned_buffer);
    ret.push_str(&self.transformer.display_string());

    ret
  }
}

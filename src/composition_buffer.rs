use crate::composition_types::CompositionType;
use crate::keycodes::KeyCode;
use crate::transformers::direct::DirectTransformer;
use crate::transformers::Transformer;

pub struct CompositionBuffer {
  transformer: Box<dyn Transformer>,
  current_composition_type: CompositionType,
  compositioned_buffer: String,
}

impl CompositionBuffer {
  pub fn new(composition_type: CompositionType) -> Self {
    CompositionBuffer {
      transformer: Self::new_transformer(&composition_type),
      current_composition_type: composition_type,
      compositioned_buffer: "".to_string(),
    }
  }

  fn new_transformer(composition_type: &CompositionType) -> Box<dyn Transformer> {
    match composition_type {
      CompositionType::Direct => Box::new(DirectTransformer::new()),
      CompositionType::Abbr => Box::new(DirectTransformer::new()),
      CompositionType::Hiragana => Box::new(DirectTransformer::new()),
      CompositionType::Katakana => Box::new(DirectTransformer::new()),
      CompositionType::EmEisu => Box::new(DirectTransformer::new()),
      CompositionType::EnKatakana => Box::new(DirectTransformer::new()),
    }
  }

  pub fn push_key(&mut self, key: &KeyCode, shift: bool) {
    self.transformer.push(key, shift);
    if !self.transformer.is_stopped() {
      return;
    }

    self
      .compositioned_buffer
      .push_str(&self.transformer.buffer_content());

    std::mem::replace(
      &mut self.transformer,
      Self::new_transformer(&self.current_composition_type),
    );
  }

  pub fn display_string(&self) -> String {
    let mut ret = "".to_string();
    ret.push_str(&self.compositioned_buffer);
    ret.push_str(&self.transformer.display_string());

    ret
  }
}

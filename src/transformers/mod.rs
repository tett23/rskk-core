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

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TransformerTypes {
  Direct,
  Hiragana,
  Katakana,
  Kanji,
  Abbr,
  EmEisu,
  EnKatakana,
}

impl TransformerTypes {
  pub fn to_transformer(&self) -> Box<dyn Transformer> {
    match self {
      TransformerTypes::Direct => Box::new(DirectTransformer::new()),
      TransformerTypes::Kanji => Box::new(DirectTransformer::new()),
      TransformerTypes::Hiragana => Box::new(HiraganaTransformer::new()),
      TransformerTypes::Katakana => Box::new(DirectTransformer::new()),
      TransformerTypes::Abbr => Box::new(DirectTransformer::new()),
      TransformerTypes::EmEisu => Box::new(DirectTransformer::new()),
      TransformerTypes::EnKatakana => Box::new(DirectTransformer::new()),
    }
  }
}

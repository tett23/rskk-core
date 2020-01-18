mod buffer_pair;
mod buffer_pairs;
mod hiragana;

use super::BufferState;

pub use buffer_pair::BufferPair;
pub use buffer_pairs::BufferPairs;

pub fn hiragana_convert(current_buffer: &str, character: char) -> Option<Vec<BufferPair>> {
  hiragana::convert(current_buffer, character)
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum LetterType {
  Direct,
  Hiragana,
  Katakana,
  EnKatakana,
  EmEisu,
}
